use std::io::prelude::*;
use std::net::TcpStream;



static CNCHANDLE: [u8;12] = [  0xa0, 0xa0,  0xa0,  0xa0, 0x00, 0x01, 0x01, 0x01, 0x00, 0x02, 0x00,0x02 ];
//static fnHeader: [u8;8] = [ 0xa0,  0xa0,  0xa0, 0xa0, 0x00, 0x01, 0x21, 0x01 ];
//static key_systeminfo: [u8;4]= [ 0x00, 0x01, 0x00, 0x18 ];
//static key_systeminfo2: [u8;4] = [ 0x00, 0x02, 0x00, 0x18 ];
//static key_cnc_rdexeprog: [u8;4] = 	[ 0x00, 0x01, 0x00, 0x20 ];
//static DEFAULT_REQ_LENGTH: i16 = 0x1c;
//static DEFAULT_REQ_COUNT_LENGTH: i16= 2;
//static DEFAULT_TOTAL_SIZE_LENGTH: i16= 2;
//static DEFALUT_REQUEST_HEADER_LENGTH: i16= 8;

fn  main()
{
    
    let   stream = TcpStream::connect("127.0.0.1:8193").unwrap();
    let _ = connect(&stream);
    let response =  read_body_length(&stream);
    println!("{:?}",response);
    //let _ = stream.write(&[1]); // ignore the Result
    //let _ = stream.read(&mut [0; 128]); // ignore this too
} // the 
fn connect(mut stream: &TcpStream){
    let _rslt= stream.write(&CNCHANDLE);
}
fn read_body_length(mut stream: &TcpStream)-> [u8;10] {
    let mut header:[u8;10]  =[0;10];
	let _rslt = stream.read(&mut header);
	header
}