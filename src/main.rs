use std::io::prelude::*;
use std::net::TcpStream;


//extern crate bincode;
//#[macro_use]
//extern crate serde_derive;

static CNCHANDLE: [u8;12] = [  0xa0, 0xa0,  0xa0,  0xa0, 0x00, 0x01, 0x01, 0x01, 0x00, 0x02, 0x00,0x02 ];
static FN_HEADER: [u8;8] = [ 0xa0,  0xa0,  0xa0, 0xa0, 0x00, 0x01, 0x21, 0x01 ];
//static key_systeminfo: [u8;4]= [ 0x00, 0x01, 0x00, 0x18 ];
//static key_systeminfo2: [u8;4] = [ 0x00, 0x02, 0x00, 0x18 ];
//static key_cnc_rdexeprog: [u8;4] = 	[ 0x00, 0x01, 0x00, 0x20 ];
//static DEFAULT_REQ_LENGTH: i16 = 0x1c;
static DEFAULT_REQ_COUNT_LENGTH: i16= 2;
//static DEFAULT_TOTAL_SIZE_LENGTH: i16= 2;
//static DEFALUT_REQUEST_HEADER_LENGTH: i16= 8;
static DEFAULT_TOTAL_SIZE_LENGTH: i16 =2;
static DEFAULT_REQ_LENGTH: i16 =0x1c;
static DEFALUT_REQUEST_HEADER_LENGTH: i16 =8;
static KEY_CNC_RDEXEPROG: [u8;4] = 	[ 0x00, 0x01, 0x00, 0x20 ];


struct Request{
     a:i16,
    b: i16
}
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

fn  main()
{
    let r = Request { a:1,b:1};
    let bytes : &[u8] = unsafe{ any_as_u8_slice(&r)};
    println!("{:?}", bytes);
    let   stream = TcpStream::connect("127.0.0.1:8193").unwrap();
    let _ = connect(&stream);
    let response =  read_body_length(&stream);
    
    let response2 = make_request_packet(1);
    
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
fn get_program(){
    let mut response = make_request_packet(1);
    response.append(KEY_CNC_RDEXEPROG.iter().cloned());
}
fn make_request_packet(count : i16) -> Vec<u8>  {
    let total_len = DEFAULT_REQ_COUNT_LENGTH + DEFAULT_REQ_LENGTH * count;
    let len = DEFALUT_REQUEST_HEADER_LENGTH + DEFAULT_TOTAL_SIZE_LENGTH + total_len;
    let mut header = vec![0; 0];
    header.extend(FN_HEADER.iter().cloned());
    header.push(total_len as u8);
    header.push(count as u8);
    unsafe {
        header.set_len(len as usize);
    }
    println!("make_request_packet {:?}  : length {:?}",header ,header.len());
	header
}