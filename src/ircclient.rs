// src/ircclient.rs
use std::fmt;
use std::io::net::tcp::TcpStream;
use std::io::net::ip::{IpAddr, SocketAddr};
use std::io::net::addrinfo::get_host_addresses;
use std::io::buffered::BufferedStream;

pub struct IrcClient {
  nick: ~str,
  user: ~str,
  priv stream: BufferedStream<TcpStream>,
  verbose: bool
}

impl IrcClient {
  fn new(nick: ~str, stream: BufferedStream<TcpStream>) -> IrcClient {
    IrcClient { nick: nick.clone(), user: nick, stream: stream, verbose: false }
  }

  fn connect(nick: ~str, host: IpAddr, port: u16) -> IrcClient { 
    let socket = SocketAddr { ip: host, port: port };
    let stream = BufferedStream::new(TcpStream::connect(socket).unwrap());
    let mut client = IrcClient::new( nick, stream );
   
    write!(&mut client.stream as &mut Writer, "NICK {}\r\n", client.nick)
    write!(&mut client.stream as &mut Writer, "USER {} 8 * :{}\r\n", client.nick, client.user);
    
    client.stream.flush();
    client
  }

  fn listen(&mut self) {
    loop {
      let msg = self.stream.read_line();
      match msg {
        Some(msg) => {

            let input: ~[&str] = msg.split(':').collect();
             
            for i in input.iter() {
              print(*i);
            }
            
            if input[0].contains("PING") {
              let response: &str = format!("PONG {}\r\n", input[1]);
              print(response);
              self.stream.write(response.as_bytes());
              self.stream.flush();
            }

            if input[1].contains("004") {
              self.join("#rustbot-test");
            }
        },
        None => ()
        }
      }
    }

  fn join(&mut self, channel: &str) {
    println(format!("JOIN {}\r\n", channel));
    write!(&mut self.stream as &mut Writer, "JOIN {}\r\n", channel);
    self.stream.flush();
  }
}

impl fmt::Default for IrcClient {
    fn fmt(obj: &IrcClient, f: &mut fmt::Formatter) {
      write!(f.buf, "nick: {}, name: {}", obj.nick, obj.user);
    }
}

fn main() {
  // let hostname: ~str = ~"irc.mozilla.org", port = 6667;
  let hostname = ~"localhost";
  let port = 6667;
  // get_host_addresses returns an Option<vector>
  let host = get_host_addresses(hostname).unwrap();
  let mut client = IrcClient::connect(~"rustbot", host[0], port);
  client.listen();
}
