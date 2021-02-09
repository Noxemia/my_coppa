
struct Message{
    content: String
}

trait Communication{
    fn recive(&self, message: Message);
}

struct Port{
    recipient:  Box<dyn Communication>
}

impl Port{
    fn new(recipient: Box<dyn Communication>) -> Self{
        Self{
            recipient
        }

    }
}

struct Website{
    
}

impl Website {

    fn new() -> Self{
        Self{}
    }

    fn done(&self, message: Message) {
        println!("Website prints the message: {}", message.content)
    }
}

impl Communication for Website{

    fn recive(&self, message: Message) {
        &self.done(message);
    }
}


struct Parent{
    interface: Box<Port>
}

impl Parent {
    fn new(interface: Box<Port>) -> Self{
        Self{
            interface
        }
    }

    fn send(&self, message: Message){
        
    }
}


fn main(){
    println!("Hello World");
    let website = Website::new();

    let port = Port::new(Box::new(website));

    let parent = Parent::new(Box::new(port));


}