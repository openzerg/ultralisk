use std::sync::{Arc, Mutex};

pub struct Subscription {
    unsubscribe: Box<dyn Fn() + Send + Sync>,
}

impl Subscription {
    pub fn new(unsubscribe: Box<dyn Fn() + Send + Sync>) -> Self {
        Self { unsubscribe }
    }

    pub fn unsubscribe(&self) {
        (self.unsubscribe)();
    }
}

pub trait Observer<T>: Send + Sync {
    fn next(&self, value: T);
    fn error(&self, err: &(dyn std::error::Error + Send + Sync)) {
        let _ = err;
    }
    fn complete(&self) {}
}

pub struct FnObserver<T> {
    next_fn: Box<dyn Fn(T) + Send + Sync>,
    error_fn: Option<Box<dyn Fn(&(dyn std::error::Error + Send + Sync)) + Send + Sync>>,
    complete_fn: Option<Box<dyn Fn() + Send + Sync>>,
}

impl<T> FnObserver<T> {
    pub fn new<F: Fn(T) + Send + Sync + 'static>(f: F) -> Self {
        Self {
            next_fn: Box::new(f),
            error_fn: None,
            complete_fn: None,
        }
    }
}

impl<T: Send + Sync> Observer<T> for FnObserver<T> {
    fn next(&self, value: T) {
        (self.next_fn)(value);
    }

    fn error(&self, err: &(dyn std::error::Error + Send + Sync)) {
        if let Some(f) = &self.error_fn {
            f(err);
        }
    }

    fn complete(&self) {
        if let Some(f) = &self.complete_fn {
            f();
        }
    }
}

pub struct BehaviorSubject<T> {
    current_value: Arc<Mutex<T>>,
    observers: Arc<Mutex<Vec<Box<dyn Observer<T>>>>>,
    closed: Arc<Mutex<bool>>,
}

impl<T: Clone + Send + Sync + 'static> BehaviorSubject<T> {
    pub fn new(initial_value: T) -> Self {
        Self {
            current_value: Arc::new(Mutex::new(initial_value)),
            observers: Arc::new(Mutex::new(Vec::new())),
            closed: Arc::new(Mutex::new(false)),
        }
    }

    pub fn get_value(&self) -> T {
        self.current_value.lock().unwrap().clone()
    }

    pub fn next(&self, new_value: T) {
        if *self.closed.lock().unwrap() {
            return;
        }
        *self.current_value.lock().unwrap() = new_value.clone();
        let observers = self.observers.lock().unwrap();
        for observer in observers.iter() {
            observer.next(new_value.clone());
        }
    }

    pub fn error(&self, err: &(dyn std::error::Error + Send + Sync)) {
        if *self.closed.lock().unwrap() {
            return;
        }
        {
            let observers = self.observers.lock().unwrap();
            for observer in observers.iter() {
                observer.error(err);
            }
        }
        *self.closed.lock().unwrap() = true;
        self.observers.lock().unwrap().clear();
    }

    pub fn complete(&self) {
        if *self.closed.lock().unwrap() {
            return;
        }
        let observers = self.observers.lock().unwrap();
        for observer in observers.iter() {
            observer.complete();
        }
        *self.closed.lock().unwrap() = true;
        self.observers.lock().unwrap().clear();
    }

    pub fn subscribe(&self, observer: Box<dyn Observer<T>>) -> Subscription {
        let mut observers = self.observers.lock().unwrap();
        observers.push(observer);
        let current = self.current_value.lock().unwrap().clone();
        if let Some(obs) = observers.last() {
            obs.next(current);
        }

        let observers_arc = Arc::clone(&self.observers);
        let len = observers.len() - 1;
        Subscription::new(Box::new(move || {
            let mut obs = observers_arc.lock().unwrap();
            if len < obs.len() {
                obs.remove(len);
            }
        }))
    }

    pub fn subscribe_fn<F: Fn(T) + Send + Sync + 'static>(&self, f: F) -> Subscription {
        self.subscribe(Box::new(FnObserver::new(f)))
    }
}
