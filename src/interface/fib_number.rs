
// 
#[py::class]
struct BaseClass {
   val1: usize
}

#[py::methods]
impl BaseClass {
   #[new]
   fn __new__(obj: &PyRawObject) -> PyResult<()> {
       obj.init(|t| BaseClass{val1: 10})
   }
   
   pub fn method(&self) -> PyResult<()> {
      Ok(())
   }
}

#[py::class(base=BaseClass)]
struct SubClass {
   val2: usize
}

#[py::methods]
impl SubClass {
   #[new]
   fn __new__(obj: &PyRawObject) -> PyResult<()> {
       obj.init(|t| SubClass{val2: 10})
       BaseClass::__new__(obj)
   }
   
   fn method2(&self) -> PyResult<()> {
       self.get_base().method()
   }
}
