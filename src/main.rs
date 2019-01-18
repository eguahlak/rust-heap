use std::fmt;

pub trait Queue<T> {
    fn enqueue(&mut self, element: &T);
    fn dequeue(&mut self, element: &T);
    }

pub trait PriorityQueue<T> where T: PartialOrd {
    fn insert(&mut self, element: T);
    fn top(&self) -> T;
    fn remove_top(&mut self) -> T;
    fn is_empty(&self) -> bool;
    }

pub struct Heap<T> where T: PartialOrd {
    data: Vec<T>,
    capacity: usize
    }

impl<T> Heap<T> where T: PartialOrd + fmt::Debug + std::clone::Clone {
    fn new() -> Heap<T> {
        Heap { data : Vec::new(), capacity : 0 }
        }
    fn from_vec(data: Vec<T>) -> Heap<T> {
        let mut heap = Heap { data, capacity : 0 };
        loop {
            let len = heap.data.len();
            if len == heap.capacity { break; }
            heap.swim(heap.capacity);
            heap.capacity += 1;
            println!("*** {}", heap);
            }
        heap
        }
    fn left_of(index: usize) -> usize { 2*(index + 1) - 1 }
    fn right_of(index: usize) -> usize { 2*(index + 1) }
    fn parent_of(index: usize) -> usize { (index + 1)/2 - 1 }

    fn swim(&mut self, index: usize) {
        if index == 0 { return; }
        let parent_index = Heap::<T>::parent_of(index);
        if self.data[index] >= self.data[parent_index] { return; }
        else {
            self.data.swap(index, parent_index);
            self.swim(parent_index);
            }
        }

    fn sink(&mut self, index: usize) {
        let left_index = Heap::<T>::left_of(index);
        let right_index = Heap::<T>::right_of(index);
        if left_index >= self.capacity && right_index >= self.capacity { return; }
        if left_index >= self.capacity { // no left branch
            if self.data[right_index] >= self.data[index] { return; }
            self.data.swap(right_index, index);
            self.sink(right_index);
            }
        else if right_index >= self.capacity { // no right branch
            if self.data[left_index] >= self.data[index] { return; }
            self.data.swap(left_index, index);
            self.sink(left_index);
            }
        else { // both branches exist
            if self.data[left_index] < self.data[right_index] { // going left
                if self.data[left_index] >= self.data[index] { return; }
                self.data.swap(left_index, index);
                self.sink(left_index);
                }
            else { // going right
                if self.data[right_index] >= self.data[index] { return; }
                self.data.swap(right_index, index);
                self.sink(index);
                }
            }
        }
    pub fn sort(&mut self) {
        loop {
            if self.capacity == 0 { break; }
            self.remove_top();
            }
        self.capacity = self.data.len();
        }
    pub fn remove_top(&mut self) -> T {
        let top = self.data[0].clone();
        self.capacity -= 1;
        self.data.swap(0, self.capacity);
        self.sink(0);
        top
        }
    }

impl<T> PriorityQueue<T> for Heap<T> where T: PartialOrd + std::clone::Clone + std::fmt::Debug {
    fn insert(&mut self, element: T) {
        self.data.push(element);
        self.swim(self.capacity);
        self.capacity += 1;
        }
    fn top(&self) -> T {
        self.data[0].clone()
        }
    fn remove_top(&mut self) -> T { self.remove_top() }
    fn is_empty(&self) -> bool { self.capacity == 0 }
    }

impl<T> fmt::Display for Heap<T> where T: PartialOrd + std::fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "data: {:?}\ncapacity: {}", self.data, self.capacity)
        }
    }


fn test1() {
    let mut heap = Heap::<i32>::new();
    heap.insert(20);
    heap.insert(5);
    heap.insert(35);
    heap.insert(2);
    heap.insert(95);
    heap.insert(22);
    heap.insert(50);

    println!("{}", heap);
    println!("top: {}", heap.remove_top());
    println!("{}", heap);
    while !heap.is_empty() {
        println!("  {}", heap.remove_top());
        }
    println!("result: {}", heap);
    println!("-------");
    let heap = Heap::from_vec(vec![5, 3, 7, 1]);
    println!("--> {}", heap);

    let mut sheap = Heap::from_vec(vec!["Erik", "Bente", "Anders", "Dorte", "Christine"]);
    sheap.sort();
    println!("Names: {}", sheap);
    sheap.sort();
    println!("Names: {}", sheap);
    }

fn main() {
    test1();


    }
