extern crate byteorder;
extern crate hex_slice;
use hex_slice::AsHex;

use std::io::prelude::*;
use std::net::TcpStream;

use byteorder::{BigEndian, WriteBytesExt,ByteOrder};

//extern crate bincode;
//#[macro_use]
//extern crate serde_derive;

static CNCHANDLE: [u8;12] = [  0xa0, 0xa0,  0xa0,  0xa0, 0x00, 0x01, 0x01, 0x01, 0x00, 0x02, 0x00,0x02 ];
static FN_HEADER: [u8;8] = [ 0xa0,  0xa0,  0xa0, 0xa0, 0x00, 0x01, 0x21, 0x01 ];
//static key_systeminfo: [u8;4]= [ 0x00, 0x01, 0x00, 0x18 ];
//static key_systeminfo2: [u8;4] = [ 0x00, 0x02, 0x00, 0x18 ];
//static key_cnc_rdexeprog: [u8;4] = 	[ 0x00, 0x01, 0x00, 0x20 ];
//static DEFAULT_REQ_LENGTH: i16 = 0x1c;
static DEFAULT_REQ_COUNT_LENGTH: u16= 2;
//static DEFAULT_TOTAL_SIZE_LENGTH: i16= 2;
//static DEFALUT_REQUEST_HEADER_LENGTH: i16= 8;
static DEFAULT_TOTAL_SIZE_LENGTH: u16 =2;
static DEFAULT_REQ_LENGTH: u16 =0x1c;
static DEFALUT_REQUEST_HEADER_LENGTH: u16 =8;
static KEY_CNC_RDEXEPROG: [u8;4] = 	[ 0x00, 0x01, 0x00, 0x20 ];


fn  main()
{
    
    
    let   stream = TcpStream::connect("127.0.0.1:8193").unwrap();
    let _ = connect(&stream);
    
    let response2 = get_program(&stream);
    
    //let _ = stream.write(&[1]); // ignore the Result
    //let _ = stream.read(&mut [0; 128]); // ignore this too
} // the 
fn connect(mut stream: &TcpStream){
    let _rslt= stream.write(&CNCHANDLE);
    let r = read_body_length(stream);
    //println!("bodyLen {:?}  ",r);
    let  mut body = vec![0; r as usize];
    stream.read(&mut body);
    println!("Body {:?}\n Len {} ",body,body.len());
}
fn read_body_length(mut stream: &TcpStream)-> u16 {
    let mut header:[u8;10]  =[0;10];
	let _rslt = stream.read_exact(&mut header);
    let len = BigEndian::read_u16(&header[8..10]);
    len
}
fn get_program(mut stream: &TcpStream){
    let mut response = make_request_packet(1);
    make_request(&mut response, &KEY_CNC_RDEXEPROG, 1024);
    //println!("sendPacket {:02x}  : length {:?} : capacity {:?}",response.as_hex() ,response.len(),response.capacity());
    stream.write(&response);
    let r = read_body_length(stream);
    //println!("bodyLen {:?}  ",r);
    let  mut body = vec![0; r as usize];
    stream.read(&mut body);
    println!("Body {:?}\n Len {} ",body,body.len());
    cnc_rdexecprog(&mut body);
}
fn make_request_packet(count : u16) ->  Vec<u8>  {
    let total_len = DEFAULT_REQ_COUNT_LENGTH + DEFAULT_REQ_LENGTH * count;
    let len = DEFALUT_REQUEST_HEADER_LENGTH + DEFAULT_TOTAL_SIZE_LENGTH + total_len;
    let mut header = Vec::with_capacity(len as usize);
    header.extend(FN_HEADER.to_vec());
    header.write_u16::<BigEndian>(total_len).unwrap();
    header.write_u16::<BigEndian>(count).unwrap();
    //println!("make_request_packet {:?}  : length {:?} : capacity {:?}",header ,header.len(),header.capacity());
	header
}

fn make_request( vec :&mut Vec<u8> , key : &[u8] , param : i32) {
    let size = vec.capacity();
    vec.write_u16::<BigEndian>(DEFAULT_REQ_LENGTH).unwrap();
    vec.write_u16::<BigEndian>(1).unwrap();
    vec.extend(key.to_vec());
    vec.write_i32::<BigEndian>(param).unwrap();
    vec.resize(size,0);
    
}
fn cnc_rdexecprog(vec : &mut Vec<u8> ){
    let result_count = BigEndian::read_u16(&vec);
    println!("resultCount {}",result_count);
}