#![feature(nll)]
use ::std::cell::RefCell;
use ::std::rc::Rc;
use ::std::fmt::Display;
use ::std::fmt;
use ::std::cell::RefMut;
use ::std::cell::Ref;
use ::std::convert;
use ::std::iter::Iterator;

pub struct LkdLt<'a, T: 'a> {
    size: u32,
    head: Option<Rc<RefCell<Node<T>>>>,
    now_iter: Option<&'a Option<Rc<RefCell<Node<T>>>>>,
}

pub struct Node<T> {
    data: RefCell<T>,
    next_node: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data_: T) -> Node<T>
    {
        Node {
            data: RefCell::new(data_),
            next_node: None,
        }
    }
}


impl<'a, T> LkdLt<'a, T>
    where T: 'a + fmt::Debug + Clone
{
    pub fn new() -> LkdLt<'a, T>
    {
        LkdLt {
            size: 0,
            head: None,
            now_iter: None,
        }
    }
    pub fn add(&mut self, val: T)
    {
        self.size = self.size + 1;
        let mut ne = Rc::new(RefCell::new(Node::new(val)));

        let mut a = &mut self.head;

        while let &mut Some(ref n) = a {
            a = unsafe{&mut (*(*n).as_ptr()).next_node};
        }
        *a = Some(ne);
        if self.size == 1 {
            self.now_iter = unsafe{Some(&(* (a as  *mut Option<Rc<RefCell<Node<T>>>>) )) };
        }
    }
    pub fn getSize(&self) -> u32 {
        self.size
    }
    pub fn get(&self, index: u32) -> Result<&T, ()>
    {
        if !(index < self.size) {
            return Err(());
        }
        let mut c: *mut Node<T>;
        let mut a: *const Option<Rc<RefCell<Node<T>>>>;
        unsafe {
            a = &self.head as *const Option<Rc<RefCell<Node<T>>>>;
            for i in 0..index {
                match *a {
                    Some(ref nn) => a = &nn.borrow_mut().next_node as *const Option<Rc<RefCell<Node<T>>>>,
                    None => break
                }
            }
            if let Some(ref n) = *a {
                c = n.as_ptr() as *mut Node<T>;
                return Ok((*c).data.get_mut());
            } else {
                return Err(());
            }
        }
    }
    pub fn set(&mut self, index: u32, val: T) -> Result<(), ()>
    {
        if !(index < self.size) {
            return Err(());
        }
        let mut a: *mut Option<Rc<RefCell<Node<T>>>>;
        let mut b: *mut T;

        unsafe {
            a = &mut self.head;

            for i in 0..index {
                match *a {
                    Some(ref nn) => a = &mut (nn.borrow_mut().next_node) as *mut Option<Rc<RefCell<Node<T>>>>,
                    None => break
                }
            }
            if let Some(ref n) = *a {
                b = n.borrow_mut().data.as_ptr();
                *b = val;
                return Ok(());
            } else {
                return Err(());
            }
        }
    }
    pub fn remove(&mut self, index: u32) -> Result<RefCell<T>, ()>
    {
        if !(index < self.size) {
            return Err(());
        }
        let mut a: *mut Option<Rc<RefCell<Node<T>>>>;
        let mut last: *mut Node<T>;
        let mut mid: Rc<RefCell<Node<T>>>;
        let mut next: Rc<RefCell<Node<T>>>;


        unsafe {
            a = &mut self.head as *mut Option<Rc<RefCell<Node<T>>>>;

            if index == 0 {
                if let Some(ref n) = *a {
                    mid = Rc::clone(n);
                    a = &mut (n.borrow_mut().next_node) as *mut Option<Rc<RefCell<Node<T>>>>;

                    if let Some(ref nn) = *a {
                        next = Rc::clone(nn);
                        self.head = Some(next);
                    } else {
                        //说明 只有一个
                        self.head = None;
                    }
                } else {
                    return Err(());
                }
                self.size = self.size - 1;
                return Ok(mid.borrow_mut().data.clone());
            }

            for i in 0..(index - 1) {
                match *a {
                    Some(ref nn) => a = &mut (nn.borrow_mut().next_node) as *mut Option<Rc<RefCell<Node<T>>>>,
                    None => break
                }
            }
            if let Some(ref n) = *a {
                last = n.as_ptr();
                a = &mut (n.borrow_mut().next_node) as *mut Option<Rc<RefCell<Node<T>>>>;
                if let Some(ref nn) = *a {
                    mid = Rc::clone(nn);
                    a = &mut (nn.borrow_mut().next_node) as *mut Option<Rc<RefCell<Node<T>>>>;
                    if let Some(ref nnn) = *a
                        {
                            next = Rc::clone(nnn);
                            (*last).next_node = Some(next);
                        } else {
                        //下一个 没有值  直接删
                        (*last).next_node = None;
                    }
                } else {
                    //中间一个没有 值  就是  错误
                    return Err(());
                }
            } else {
                //前一个 没有 值  就是  错误
                return Err(());
            }
            self.size = self.size - 1;
            return Ok(mid.borrow_mut().data.clone());
        }
    }
}


impl<'a, T: Display> Display for LkdLt<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut a: *const Option<Rc<RefCell<Node<T>>>>;
        let mut b: *const T;
        unsafe {
            a = &(self.head) as *const Option<Rc<RefCell<Node<T>>>>;

            write!(f, "[ ");
            loop {
                match *a {
                    Some(ref nn) => {
                        b = nn.borrow_mut().data.as_ptr() as *const T;
                        if let None = nn.borrow_mut().next_node {
                            write!(f, "{}", *b);
                        }else{
                            write!(f, "{},", *b);
                        }
                        a = &(nn.borrow_mut().next_node) as *const Option<Rc<RefCell<Node<T>>>>;
                    }
                    None => {
                        write!(f, " ]");
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

impl<'a, T: 'a> Iterator for LkdLt<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        let mut a : &'a Rc<RefCell<Node<T>>>;
        let mut b :  RefMut<Node<T>>;
        let mut c : * mut T;
        unsafe {
            let mut refno: *const Option<Rc<RefCell<Node<T>>>>;
            match self.now_iter {
                None => return None,
                Some(n) => refno = n as *const Option<Rc<RefCell<Node<T>>>>
            }

            match *refno {
                Some(ref nn) => {
                    refno = &nn.borrow_mut().next_node as *const Option<Rc<RefCell<Node<T>>>>;
                    self.now_iter = Some(&(*refno));
                    a = nn;
                    b = a.borrow_mut();
                    c = b.data.as_ptr();
                    return Some(&(*c));
                }
                None => {
                    refno = &self.head as *const Option<Rc<RefCell<Node<T>>>>;
                    self.now_iter = Some(&(*refno));
                    return None;
                }
            }
        }
    }
}
