fn CF_converstions(c:f64)->f64{
    return (c*1.8)+32.0;
   }
fn FC_converstions(f:f64)->f64{
    return (f-32.0)*(5.0/9.0);
}
fn main() {
    let c=31.0;
    let f= CF_converstions(c);
    println!("Celsius to Fahrenheit for 30 = {}", CF_converstions(c));
    println!("Fahrenheit to Celsius for 86 = {}", FC_converstions(f));
}
