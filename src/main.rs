fn main() {
    //variable
   //let x = 20;  // jadi bahasa pemrograman rust secara default itu bersifat immutable 
   //let mut y = 23; // kalo kita ingin variable nya mutable kita tambahka mut
   //                    // setelah penambahan variable

   //const  harga_tertinggi: u32 = 23_000; //kita bisa menambahkan variable lain berbentuk const namun perlu 
   //                                  // memerlukan data type seperti u32

   //let b = 10;
   //let b ="ten";   // ini adalah contoh variable shadowing dengan mengubah variable awal  dengan 
   //                // menulis ulang setelahnya lalu ubah nilai nua

   ////tipe data
   ////di bahasa pemrograman rust ada dua tipe data yang terdiri dari skalar type dam compond types

   ////https://doc.rust-lang.org/book/ch03-02-data-types.html


   //// skalar type = integer, floating number,boolean, character
   //let r : u16  = 12; //integer
   //let f : f32 = 2.1; // float number
   //let valid : bool = true;
   //let invalid  = false;
   //let t ='d'; // character hint harus double quotes kalo duo quetes akan dianggap integer

   //// compund types : tuples dan array
   //    // compund types adalah tipe data yang memungkinkan kita membuat variable yang didalamnya
   //    // terdapat  mameliki  berbagai nilai dan tipe dalam satu variable 
   //let tup:(i32,f64,u8) = (100,1.3,1); //ini setelah di deklarasi tak bisa diubah

   //// cara mengakses nya dengan destructur
   //let(x,y,z) = tup;
   //// cara mengakses menggunakan index
   //let first = tup.0;
   //// ada juga penulisan unique value 
   //let tup2 = ();

   //let a = [1,2,3]; // ini adalah array sama seperti tuples sekali di deklarasi tidak bisa diubah
   //let d: [i32; 5] = [1,2,3,4,5]; // mereke harus setipe beda dengan tuples, 
   //let f: [i32; 2] = [1,2];          //dalam satu variable harus satu type data
   //let f = [3,5]; // -> let f =[3,3,3,3,3]

   //// cara mengakses array
   //let first = d [0];

   //// cara kita membuat fungsi lain kita bisa di dalem atau diluar body scope misal
    //println!("halooo");
    //my_function(3,'h');

    
    
    //let num = 6;

    //if num <3{
       // println!("kondisi sesuai");
   // }else if num % 3== 0{
       // println!("kondisi berhasil di sisain");
   // }else{
       // println!("kondisi gak sesuai ")
   // }

    //let cond = true;
    //let num2 = if cond {5} else {6};

    /* contoh looping
    loop{
        println!("Looping berjalan...")
        
    }
    
    let mut counter=0;
    let res =loop{
        counter += 1;

        if counter ==10{
            break counter*2

        }
    }; 
    println!("{}",res);
    
    let mut num = 3;
    while num != 0{
        println!("{}",num);
        num -= 1;
    }
    println!("dadahhhh");
    

    let a = [1,2,3,4];

    for element in a{
        println!("jadi nilai nya adalah{}",element);

    }
    */

    for number in 1..=10{
        println!("{}",number);
    }
    println!("dadahhhh....")
}

//fn my_function(){
// println!("Result");
//}


// jika ingin menambah parameter
//fn my_function(value: i32, label: char){
    //println!("Result {}{}", value,label); // {} merupaka place holder
    //let res = valid_function(1,2);
//println!("{}",res);
//}

//fn valid_function(x: i32, y: i32)-> i32 {
   //x + y
//}

// inline comment here

/*
    block comment
*/
