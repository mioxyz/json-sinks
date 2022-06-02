#![allow(dead_code, unused_variables, unused_imports, unused_assignments)]
use regex::Regex;
use std::process::{Command, Stdio};
// use std::collections::HashMap;

use serde_json::Result;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Property<'a> {
   key:&'a str,   // TODO ideally we want to give structure to things such as: device.product.name = "asdlkj"
   value:&'a str, // instead of {key: "device.product.name", value: "asd" }
}

#[derive(Serialize, Deserialize, Debug)]
struct Sink<'a> {
   id:i32,
   state:&'a str,
   name:&'a str,
   description:&'a str,
   driver:&'a str,
   sample_specification:&'a str,
   channel_map:&'a str,
   owner_module:i32,
   mute:bool,
   volume:&'a str, //TODO vec<&'a str>
   base_volume:&'a str,
   monitor_source:&'a str,
   latency:&'a str,
   flags:Vec<&'a str>,
   properties:Vec<Property<'a>>,
   ports:&'a str, // TODO
   active_port:&'a str,
   formats:Vec<&'a str>,
}

impl<'a> Sink<'a> {
   pub fn new(raw:&'a str) -> Self {
      Self {
         id: -1,
         state: &raw[0..0],
         name: &raw[0..0],
         description: &raw[0..0],
         driver: &raw[0..0],
         sample_specification: &raw[0..0],
         channel_map: &raw[0..0],
         owner_module: -1,
         mute:false,
         volume:&raw[0..0], // TODO
         base_volume: &raw[0..0],
         monitor_source: &raw[0..0],
         latency: &raw[0..0],
         flags: Vec::new(), //Vec<&'a str>,
         properties: Vec::new(), //Vec<Property<'a>>,
         ports:&raw[0..0], // TODO
         active_port:&raw[0..0],
         formats:Vec::new(),
     }
   }
}

/*
   #[derive(Serialize, Deserialize, Debug)]
   struct Address<'a> {
      street: &'a str,
      city: String,
   }
*/

/*
   fn print_an_address() -> Result<()> {
      // Some data structure.
      let address:String = String::from("some street");

      let address = Address {
         street: &address,
         city: "London".to_owned(),
      };

      // Serialize it to a JSON string.
      let j = serde_json::to_string(&address)?;

      // Print, write to a file, or send to an HTTP server.
      println!("{}", j);

      Ok(())
   }
*/

/* struct DataSet<'a> {
   elems:HashMap<&'a str, &'a str>,
   properties:HashMap<&'a str, &'a str>,
   ports:HashMap<&'a str, &'a str>,
   formats:HashMap<&'a str, &'a str>,
// actually parsed stuff
   mute:bool,
   flags:Vec<&'a str>,
}*/


fn get_raw_sinks() -> String {
   let result = Command::new("pactl")
      .args(["list", "sinks"])
      // Tell the OS to record the command's output
      .stdout(Stdio::piped())
      // execute the command, wait for it to complete, then capture the output
      .output()
      // Blow up if the OS was unable to start the program
      .unwrap();
   // extract the raw bytes that we captured and interpret them as a string
   String::from_utf8(result.stdout).unwrap()
}

/* fn partition_sinks(mut raw: &str) -> Vec<&str>
{
   let mut sinks = Vec::new();
   loop {
      let pattern = Regex::new(r"([a-zA-Z_][a-zA-Z0-9]*)|([0-9]+)|(\.)|(=)").unwrap();

      let maybe = pattern.find_iter(raw).next();

      match maybe {
         None => println!("do somehting"),
         Some(m) => m.start()


      }

      // for m in matches
      // {
      //    m.
      //    /*
      //       let index = cap.iter().enumerate()
      //          .skip(1)              // skip the first group
      //          .find(|t| t.1.is_some())  // find the first `Some`
      //          .map(|t| t.0)          // extract the index
      //          .unwrap_or(0);         // get the index
      //       println!("group {:?}, match {:?}", index, cap.at(index).unwrap());
      //    */
      // }


      let (head, tail) = raw.split_at(1);
      raw = head;
      sinks.push(head);

      if tail.len() == 0 {
         break;
      }
   }


   sinks
}*/

fn partition_sinks(raw: &str) -> Vec<&str> {
   Regex::new(r"\nSink #").unwrap().split(raw).collect()
}

/*
   line "cells" are delimited by »\n\t*«
   If »\n\t* « then we are dealing with a
   continued line like with the Volume property.

   ..So I guess it makes sense to first split up the
   sink by the normal »\n\t*« delimiter, and then
   look if we have a continued line while iterating
   over lines.

   Normal "lines are separated like »KEY:VALUE«
   and Props are separated like »KEY1.KEY2 = "VALUE"«
   So we need to switch to a Props parser? Or...
*/

fn parse_normal_line(raw: &str) -> Vec<&str> {
   // "   Properties:"
   // Regex::new(r"\n\t*").unwrap().split(raw).collect()
   Regex::new(r"\n\t").unwrap().split(raw).collect()
}

fn main() -> Result<()> {
   // print_an_address().unwrap();

   let raw = get_raw_sinks();
   let mut sinks:Vec<Sink> = Vec::new();
   // println!("{}", raw );
   let parts = partition_sinks(&raw);
   println!("sinks length: {}", parts.len());
   let mut firstSink:bool = true; // TODO super ugly, make beautiful

   for raw_sink in parts {
      sinks.push(Sink::new(raw_sink));
      // println!("SINK PART: {}", raw_sink);
      let lines = parse_normal_line(raw_sink);
      let mut mode = 0; // TODO
      let mut parsedSinkNumber:bool = false; // TODO 
      // let mut prev:i32 = 0;
      // let mut vecMode = 0; //properties = 0, ports = 1, formats = 2
      for line in lines.iter() 
      {
         let fc = &line[0..1]; //TODO compare single char instead i.e. line[0] == '\t' or something

         if !parsedSinkNumber {
            if firstSink {           
               println!("FIRST SINK line: »{}«", line);
               let k = line.find('#').unwrap();
               println!("parsing id, »{}«", &line[(k+1)..]);          
               sinks.last_mut().unwrap().id = String::from( &line[(k+1)..])
               .trim()
               .parse::<i32>()
               .expect("can't parse owner module as int");
               firstSink = false;
            }else{
               sinks.last_mut().unwrap().id = String::from( &line[0..] )
               .trim()
               .parse::<i32>()
               .expect("can't parse owner module as int");
            }
            parsedSinkNumber = true;
            continue;
         }

         // prev = mode;
         if fc == "\t" {
            mode = 1;
         } else if fc == " " {
            mode = 2;
            // line continuation
         } else {
            mode = 0;
         }
         if mode == 0 {
            println!("> line: »{}«", line);
            let separator = line.find(':').unwrap();
            let key = &line[0..separator];
            if key == "State" {
               sinks.last_mut().unwrap().state = &line[(separator+1)..];
            } else if key == "Name" {
               sinks.last_mut().unwrap().name = &line[(separator+1)..];
            } else if key == "Description" {
               sinks.last_mut().unwrap().description = &line[(separator+1)..];
            } else if key == "Driver" {
               sinks.last_mut().unwrap().driver = &line[(separator+1)..];
            } else if key == "Sample Specification" {
               sinks.last_mut().unwrap().sample_specification = &line[(separator+1)..];
            } else if key == "channel_map" {
               sinks.last_mut().unwrap().channel_map = &line[(separator+1)..];
            } else if key == "owner_module" {
               sinks.last_mut().unwrap().owner_module = String::from( &line[(separator+1)..])
                  .trim()
                  .parse::<i32>()
                  .expect("can't parse owner module as int");
            } else if key == "Mute" {
               if "yes" == &line[(separator+1)..] {
                  sinks.last_mut().unwrap().mute = true;
               }
            }
         }
      } // --- for lines
      println!("+++ PARSED ALL LINES!");
   } // --- for parts

   for sink in sinks.iter() {
      // Serialize it to a JSON string.
      let json = serde_json::to_string(&sink)?;

      println!("{}", json);
   }

   Ok(()) // we use this Result return val because of the »?« used in the to_string method.
} // --- main
