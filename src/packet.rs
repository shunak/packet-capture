pub trait GettableEndPoints{
    fn get_source(&self) -> String;
    fn get_destination(&self) -> String;
    fn get_payload(&self) -> &[u8];
}


// implement method for Ipv4
impl<'a> GettableEndPoints for Ipv4Packet<'a> {
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }

    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }

    fn get_payload(&self) -> &[u8]{
        self.payload()
    }
}






