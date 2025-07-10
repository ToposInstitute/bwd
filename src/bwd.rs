use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub enum Bwd<T> {
    Nil,
    Snoc(Rc<(T, Bwd<T>)>),
}

impl<T> Clone for Bwd<T> {
    fn clone(&self) -> Self {
        match self {
            Bwd::Nil => Bwd::Nil,
            Bwd::Snoc(r) => Bwd::Snoc(r.clone()),
        }
    }
}

impl<T> Bwd<T> {
    pub fn snoc(&self, t: T) -> Self {
        Bwd::Snoc(Rc::new((t, self.clone())))
    }

    pub fn nil() -> Self {
        Bwd::Nil
    }

    pub fn find<F: Fn(&T) -> bool>(&self, f: F) -> Option<(usize, &T)> {
        self.find_from(0, f)
    }

    fn find_from<F: Fn(&T) -> bool>(&self, n: usize, f: F) -> Option<(usize, &T)> {
        match self {
            Bwd::Nil => None,
            Bwd::Snoc(r) => {
                let (x, l) = (&r.0, &r.1);
                if f(x) {
                    Some((n, x))
                } else {
                    l.find_from(n + 1, f)
                }
            }
        }
    }

    pub fn pop(&self) -> Self {
        match self {
            Bwd::Nil => Bwd::Nil,
            Bwd::Snoc(r) => r.1.clone(),
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            Bwd::Nil => None,
            Bwd::Snoc(r) => {
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
            Bwd::Nil => init,
            Bwd::Snoc(r) => f(&r.0, r.1.foldl(init, f)),
        }
    }

    pub fn len(&self) -> usize {
        let mut l = self;
        let mut len = 0;
        while let Bwd::Snoc(r) = l {
            l = &r.1;
            len += 1;
        }
        len
    }

    pub fn rev_iter<'a>(&'a self) -> BwdIter<'a, T> {
        BwdIter { bwd: self }
    }

    pub fn extend_by<I: Iterator<Item = T>>(&self, iter: I) -> Self {
        iter.fold(self.clone(), |l, x| l.snoc(x))
    }
}

impl<T: Clone> Bwd<T> {
    pub fn to_vec(&self) -> Vec<T> {
        let mut out = Vec::new();
        self.write_into(&mut out);
        out
    }

    fn write_into(&self, out: &mut Vec<T>) {
        match self {
            Bwd::Snoc(r) => {
                r.1.write_into(out);
                out.push(r.0.clone())
            }
            Bwd::Nil => {}
        }
    }
}

impl<T> FromIterator<T> for Bwd<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(Bwd::nil(), |l, v| l.snoc(v))
    }
}

pub struct BwdIter<'a, T> {
    bwd: &'a Bwd<T>,
}

impl<'a, T> Iterator for BwdIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.bwd {
            Bwd::Snoc(r) => Some(&r.0),
            Bwd::Nil => None,
        }
    }
}
