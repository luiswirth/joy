// here is a (bad) custom implementation of a future

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    // whetever or not the timer has completed
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
	let mut shared_state = self.shared_state.lock().unwrap();

	if shared_state.completed {
	    Poll::Ready(())
	} else {
	    shared_state.waker = Some(cx.waker().clone());
	    Poll::Pending
	}
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
	let shared_state = Arc::new(Mutex::new(SharedState {
	    completed: false,
	    waker: None,
	}));

	let thread_shared_state = shared_state.clone();
	thread::spawn(move || {
	    thread::sleep(duration);
	    let mut shared_state = thread_shared_state.lock().unwrap();
	    shared_state.completed = true;

	    if let Some(waker) = shared_state.waker.take() {
		waker.wake()
	    }
	});
	TimerFuture { shared_state }
	}
}

// here is a (bad) custom implementation of a executor

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}


struct Task {
    // in-progress future that should be pushed to completion
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    // handle to place the task itself back onto the task queue
    task_sender: SyncSender<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}
  
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
	let future = future.boxed();
	let task = Arc::new(Task {
	    future: Mutex::new(Some(future)),
	    task_sender: self.task_sender.clone(),
	});
	self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl ArcWake for Take {
    fn wake_by_ref(arc_self: &Arc<Self>) {
	let cloned = arc_self.clone();

	arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}

impl Executor {
    fn run(&self) {
	while let Ok(task) = self.ready_queue.recv() {
	    let mut future_slot = task.future.lock().unwrap();

	    if let Some(mut future) = future_slot.take() {
		let waker = waker_ref(&task);
		let context = &mut Context::from_waker(&*waker);

		if let Poll::Pending = future.as_mut().poll(context) {
		    *future_slot = Some(future);
		}
	    }
	}
    }
}

#[test]
fn test_custom_async() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
	println!("hello!");

	TimerFuture::new(Duration::new(2, 0)).await;
	println!("done!");
    });

    // drop the spawner so that our executor knows it is finished

    drop(spawner);

    executor.run();
}
