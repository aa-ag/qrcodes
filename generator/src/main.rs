use qrcode_generator::QrCodeEcc;

fn main() {
    let result: Vec<Vec<bool>> = qrcode_generator::to_matrix("Hello world!", QrCodeEcc::Low).unwrap();

    println!("{:?}", result);
}
