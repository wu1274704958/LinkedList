extern crate LinkedList;

use LinkedList::LkdLt;

fn main() {
    let mut a = LkdLt::new();

    a.add(7);
    a.add(18);
    a.add(27);
    a.add(38);
    a.add(47);
    a.add(58);
    a.add(668);
    a.add(79);

    match a.remove(0) {
        Ok(v) => println!("remove success {:?}",v),
        Err(()) => println!("remove default")
    }

    match a.remove(6) {
        Ok(v) => println!("remove success {:?}",v),
        Err(()) => println!("remove default")
    }

    match a.set(3,10000) {
        Ok(()) => println!("set success"),
        Err(()) => println!("set failed")
    }

    a.add(7);
    a.add(18);
    a.add(27);
    a.add(38);
    a.add(47);
    a.add(58);
    a.add(668);
    a.add(79);


    for i in 0 .. 10
    {
        println!("{:?} ",a.get(i));
    }

    match a.remove(0) {
        Ok(v) => println!("remove success {:?}",v),
        Err(()) => println!("remove default")
    }


    while let Some(n) = a.next(){
        println!("{}",n);
    }
    println!("hsjhdjahjdhasjd");
    while let Some(n) = a.next(){
        println!("{}",n);
    }
    println!("Hello, world! {} size : {}",a, a.getSize());

    let b: LkdLt<i32> = LkdLt::new();
    
    println!("b = {}",b);
}
