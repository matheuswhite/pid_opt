use std::thread::JoinHandle;

pub trait Work {
    type Input;
    type Output;

    fn work(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output>;
    fn set_id(&mut self, id: usize);
}

pub fn work_pool<I, O, W>(size: usize, mut input: Vec<I>, work: W) -> Vec<O>
where
    W: Work<Input = I, Output = O> + Send + 'static + Clone,
    I: Send + 'static,
    O: Send + 'static,
{
    let mut handles = vec![];
    let mut id = 0;
    while !input.is_empty() {
        let chunk = input.drain(..size.min(input.len())).collect::<Vec<_>>();
        let mut work = work.clone();
        work.set_id(id);
        let handle: JoinHandle<Vec<O>> = std::thread::spawn(move || work.work(chunk));
        handles.push(handle);

        id += 1;
    }

    let mut results = vec![];
    for handle in handles {
        if let Ok(result) = handle.join() {
            results.push(result);
        }
    }

    results.into_iter().flatten().collect()
}
