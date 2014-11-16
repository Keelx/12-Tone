use std::os;
use std::rand::{task_rng, Rng};
use std::num::ToPrimitive;

//converts 1-12 to the pitch classes
fn int_to_pitch(n: int) -> &'static str {
  match n {
    1  => "C    ",
    2  => "C#/Db",
    3  => "D    ",
    4  => "D#/Eb",
    5  => "E    ",
    6  => "F    ",
    7  => "F#/Gb",
    8  => "G    ",
    9  => "G#/Ab",
    10 => "A    ",
    11 => "A#/Bb",
    12 => "B    ",
    _  => "Err"
  }
}

fn int_to_length(n: int) -> f32 {
  match n {
    1 => 16.0/16.0, //whole note
    2 =>  8.0/16.0, //half note
    3 =>  4.0/16.0, //quarter note
    4 =>  2.0/16.0, //eight note
    5 =>  1.0/16.0, //sixteenth note
    _ => -1.0
  }
}

fn f32_to_length(n: f32) -> &'static str {
  match n {
    1.0    => "Whole note",
    0.5    => "Half note",
    0.25   => "Quarter note",
    0.125  => "Eigth note",
    0.0625 => "Sixteenth note",
    _      => "Err"
  }
}

//generic array summation function
fn sum<T: ToPrimitive>(n: &[T]) -> f32 {
  let mut total: f32 = 0.0;
  for i in n.iter() {
    total += ToPrimitive::to_f32(i).expect("Must be conversible!");
  }
  total
}

fn main() {

  let args = os::args();
  let mut bars: int = 8;
  let mut complex_rhythm: bool;
  if args.len() == 1 {
    println!("No valid bar count found, defaulting to 8.");
  } else {
    let bars_in = &args[1];
    bars = match from_str(bars_in.as_slice()){
      Some(n) => n,
      None => {println!("No valid bar count found, defaulting to 8."); 8}
    };
    if bars < 4 {
      println!("Must be minimum 4 bars! Defaulting to 4.");
      bars = 4;
    }
  }
  if args.len() == 2 {
    complex_rhythm = false;
    println!("Defaulting to standard rhythm.");
  } else {
    let rhythm_in = &args[2];
    if rhythm_in.as_slice() == "complex" {
      complex_rhythm = true;
      println!("Using complex rhythm.");
    } else {
      complex_rhythm = false;
      println!("Using standard rhythm.");
    }
  }

  let mut pitch = [0i, ..12]; //can be 1 - 12
  let mut length = [0.0, ..12]; //can be 0.125 - 1

  println!("Generating a 12 tone melody...");
  'pitch: loop {
    let mut scale: [int, ..12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    for i in range(0u,12) {
      //generate a pitch, make sure all 12 tones are used
      if scale[i] != 0 {
        'scale: loop {
          let rnd = task_rng().gen_range(0u, 12u) as uint;
          if pitch[rnd] == 0 {
            pitch[rnd] = scale[i];
            scale[i] = 0;
            println!("Generated the {}th note!", i);
            break 'scale;
          }
        }
      }
    }
    if sum(scale) == 0.0 { //if there are no notes in the scale left
      break 'pitch;
    }
  }

  println!("Length is {} bars, generating rhythm...", bars);
  'length: loop {
    for i in range(0u,12) {
      //generate the length
      if complex_rhythm {
        length[i] = (task_rng().gen_range(1u, 16u) as f32) / 16.0; //generates sixteenth notes
      } else {
        length[i] = int_to_length(task_rng().gen_range(1u, 5u) as int);
      }
    }
    //if the length doesn't fit in the bar count, re-loop until it does
    if sum(length) == bars as f32 {
      break 'length;
    }
    println!("Failed to fit the melody to the bars...");
  } //TODO: if it doesn't fit, add rests

  println!("Successfully generated a melody!");
  println!("");
  if complex_rhythm {
    for i in range(0, 12) {
      println!("{}  {}/16", int_to_pitch(pitch[i]), length[i] * 16.0);
    }
  } else {
    for i in range(0, 12) {
      println!("{}  {}", int_to_pitch(pitch[i]), f32_to_length(length[i]));
    }
  }
}
