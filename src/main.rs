fn CF_converstions(c:f32)->f32{
    return (c*1.8)+32.0;
   }
fn FC_converstions(f:f32)->f32{
    return (f-32.0)*(5.0/9.0);
}
fn main() {
    println!("celcius to F for 30 = {}", CF_converstions(30.0));
    println!("Fahrenheit to c for 86 = {}", FC_converstions(86.0));
}
