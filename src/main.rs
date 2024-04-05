fn takes_ownership(some_string: String) {
    println!("Received: {}", some_string); // setelah kita buat fungsi dan parameter seperti diatas 
                                            // kita buat print buat memastikan placeholder menerima
                                            // some_string hello pada parameter diatas

    // Lakukan sesuatu dengan some_string
    // Kepemilikan some_string akan dipindahkan ke dalam fungsi
}
fn makes_copy(some_integer: i32) {
    println!("Received: {}",some_integer)
    // Lakukan sesuatu dengan i
}

fn gives_ownership() -> String {
    let some_str = String::from("Za warudo");
    some_str // Menggunakan 'some_str' sebagai return value
}


fn takes_and_gives_back(c_string:String) -> String {
    c_string
}

fn calculate_length(h:String) ->(String,usize) {
    let length = h.len();
    (h, length)

}

fn calculate_length1(s: &String) -> usize { // jadi s adalah refrences
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(",kamuuu"); // langkah terakhir di parameter fungsi kita taro mut di refrences
                                            // string pada parameter
}
/** 
fn dangle() -> &String {
    let manggle = String::from("manggle oh da");
    &manggle


}// manggle sudah di drop
*/
/* 
fn dangle() -> &String {
    let manggle = String::from("manggle oh da");
    &manggle

}// manggle sudah di drop

*/

//slice
                            // awalnya parameter menggunakan usize
fn kata_pertama(z: &String) -> &str {  // menggunakan &string refrense karena kita ingin mengambil ownership
    let bytes = z.as_bytes();    // diparameter

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &z[0..i];// setelah kita ubah parameter usize ubah return menjadi 
        }
    }

    // Handle the case when no space is found
    &z[..]
}

fn main (){
    { // jadi variable s tidak valid disini, karena belum di deklarasi kan
        let s = "hello";
        
        println!("{}",s); //mulai dari ini kedepanya variable s sudah di deklarasi
                          // ayo buat sesuatu
    }// scope ini sudah berakhir, s sudah tidak valid lagi

    {
        let gro= 23;
        let bian=24;

        println!("{}",gro + bian);
    }

    {
       let mut g = String::from("Hello"); // g adalah string leteral
       g.push_str(",world");
       println!("{}",g)

    }

    {
        let x = 10;
        let y = x; // adanya copy value jadi stack mengcopy nilai pada x ke y bernama copy <= copy trait

        println!("x = {}, y ={}",x,y);

        let s1 = String::from("hallo");
        //let s2 = s1; // berpindah jadi gabisa dipanggil
        let s2 = s1.clone(); // jika ingin copy kita bisa pake clone 
        println!("s1={}, s2 ={}", s1, s2);
    }

    {
        let a = String::from("Haloooo"); 

        takes_ownership(a); // agar bisa menggunakan takes ownership kita buat fungsi dliuar
                                        // fungsi main
        let i = 45;
        makes_copy(i);
    }

    {
        let s1 = gives_ownership();
        println!("{}",s1);

        let s2 = String::from("hallo in alan");

        let s3 = takes_and_gives_back(s2);
        println!("{}", s3)
    }

    {
        let s3 = String::from("SATRIA");
        let (s4, len) = calculate_length(s3);
        println!("the length of {} is {}",s4,len);
    }

    // Refrence , Borrowing, Slice 

    {
        let sas1 = String::from("Alan");
        let len = calculate_length1(&sas1); // jadi & adalah meminjam ownership nya sas1
        println!(" jadi huruf pada {}, berjumlah {}", sas1,len) // & adalah lambang refrence
                                                                // bersifat meminjam dan takan mengambil
                                                                // ownership
                                                    // refrence bersifat immutable
    }

    {
        let mut l = String::from("haii haii"); // karena bersifat immutable jadinya di fungsi some
        change( &mut l)                    // string error cara mengataasinya adalah kita ubah
                                                    // variable l mejadi mutable dengan menambahkan mut


    }
    // misal
    {
      /*  let mut agi = String::from("gantengg bangesii");
        let najwa = & mut agi;
        let zahratunissa = & mut agi;
        print!("jadi pacar aku itu{},{}",najwa,zahratunissa) 

            seperti ini tidak bisa jadi hilangkan mut nya
        
        */ 

        let agi = String::from("gantengg");
        let najwa = &  agi;
        let zahratunissa = &agi;
        println!("pacar aku itu {}, ayang aku {}",najwa,zahratunissa) 
    }
    
    {   // cara mencampurkan mutable dan immutable
        /*
        let mut al = String::from("oy oy");
        let jet1 = &al;
        let jet2 = &al;
        //let jet3 = &mut al;

        println!("{}, {}",jet1,jet2);  // ini gabisa 
        
        */
        let mut lan = String::from("oy oy");
        let sp1 = &lan; 
        let sp2 = &lan;
        let sp3 = &lan;

        println!("{}, {}, {}",sp1,sp2,sp3);

        let sp4 =&mut lan; // ini bisa caranya karena mutable itu punya scope sendiri

        println!("{}",sp4)
        
    }
    /* 
    {
        let reff  = dangle();
    }
    */
    {
        let mut contoh = String::from("Hallo sekai");
        let kata = kata_pertama(&contoh);

       // contoh.clear(); // akan mengkosongkan string == " "

        // variable kata masing mempunyai nilai 5

        println!("{}",kata)
    }

    {
        let mut contoh2 = String::from("lollo Sekai");

        let lollo = &contoh2[..=5];
        let Sekai = &contoh2[6..];

        println!("{}",lollo);
        println!("{}",Sekai);
    }
}