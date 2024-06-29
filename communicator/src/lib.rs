pub mod client;

pub mod network; //{
    //fn connect() {
    
    //}
    //mod client {
        //fn connect() {                                    //This can be accessed network::client::connect()	
            
        //}
    //}
    //mod server {
        //fn connect(){
            
        //}
    //}
//}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
