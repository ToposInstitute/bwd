use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub enum Bwd1<T> {
    Singleton(Rc<T>),
    Snoc(Rc<(T, Bwd1<T>)>),
}

impl<T> Clone for Bwd1<T> {
    fn clone(&self) -> Self {
        match self {
            Bwd1::Singleton(x) => Bwd1::Singleton(x.clone()),
            Bwd1::Snoc(r) => Bwd1::Snoc(r.clone()),
        }
    }
}

impl<T> Bwd1<T> {
    pub fn snoc(&self, t: T) -> Self {
        Bwd1::Snoc(Rc::new((t, self.clone())))
    }

    pub fn singleton(t: T) -> Self {
        Bwd1::Singleton(Rc::new(t))
    }

    pub fn find<F: Fn(&T) -> bool>(&self, f: F) -> Option<(usize, &T)> {
        self.find_from(0, f)
    }

    fn find_from<F: Fn(&T) -> bool>(&self, n: usize, f: F) -> Option<(usize, &T)> {
        match self {
            Bwd1::Singleton(x) => {
                if f(x) {
                    Some((n, x))
                } else {
                    None
                }
            }
            Bwd1::Snoc(r) => {
                let (x, l) = (&r.0, &r.1);
                if f(x) {
                    Some((n, x))
                } else {
                    l.find_from(n + 1, f)
                }
            }
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            Bwd1::Singleton(x) => {
                if i == 0 {
                    Some(&x)
                } else {
                    None
                }
            }
            Bwd1::Snoc(r) => {
                if i == 0 {
                    Some(&r.0)
                } else {
                    r.1.get(i - 1)
                }
            }
        }
    }

    pub fn foldl<S, F: Fn(&T, S) -> S>(&self, init: S, f: &F) -> S {
        match self {
            Bwd1::Singleton(x) => f(x, init),
            Bwd1::Snoc(r) => f(&r.0, r.1.foldl(init, f)),
        }
    }

    pub fn len(&self) -> usize {
        let mut l = self;
        let mut len = 0;
        while let Bwd1::Snoc(r) = l {
            l = &r.1;
            len += 1;
        }
        len + 1
    }

    pub fn rev_iter<'a>(&'a self) -> Bwd1Iter<'a, T> {
        Bwd1Iter { bwd: Some(self) }
    }

    pub fn extend_by<I: Iterator<Item = T>>(&self, iter: I) -> Self {
        iter.fold(self.clone(), |l, x| l.snoc(x))
    }
}

impl<T: Clone> Bwd1<T> {
    pub fn to_vec(&self) -> Vec<T> {
        let mut out = Vec::new();
        self.write_into(&mut out);
        out
    }

    fn write_into(&self, out: &mut Vec<T>) {
        match self {
            Bwd1::Snoc(r) => {
                r.1.write_into(out);
                out.push(r.0.clone())
            }
            Bwd1::Singleton(x) => out.push((**x).clone()),
        }
    }
}

pub struct Bwd1Iter<'a, T> {
    bwd: Option<&'a Bwd1<T>>,
}

impl<'a, T> Iterator for Bwd1Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.bwd {
            Some(Bwd1::Snoc(r)) => {
                self.bwd = Some(&r.1);
                Some(&r.0)
            }
            Some(Bwd1::Singleton(x)) => {
                self.bwd = None;
                Some(x)
            }
            None => None,
        }
    }
}
