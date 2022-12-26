fn main(){

}

struct Test{
    test: bool
}

impl<'c, 'a : 'c, 'b : 'c> Test{
    fn get<T>(&self, vec_1: &'a Vec<T>, vec_2: &'b Vec<T>) -> &'c T{
        match self.test {
            true => vec_1.first().unwrap(),
            false => vec_2.first().unwrap()
        }
    }


}

