#![feature(float_to_from_bytes)]

use chrono::{Datelike, Timelike, Utc, Local};
use std::str;
use std::fs::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use std::borrow::BorrowMut;
use cursive::event::Key;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, MouseButton, MouseEvent};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::vec::Vec2;
use cursive::views::*;
use cursive::view::*;
use cursive::{Cursive, With};
use cursive::Printer;
use cursive::align::{HAlign, Align};
use cursive::view::SizeConstraint;
use std::io::SeekFrom;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut siv = Cursive::default();
    let mut last_number_used = String::new();
    let mut number_of_items = String::new();
    let file = open_file( &mut last_number_used,  &mut number_of_items);
    get_inv_header(&mut last_number_used, &mut number_of_items);
    println!("\nLast number: {}\n# of items: {}", last_number_used, number_of_items);
    siv.add_layer(
        Dialog::new()
            .title("MiniStore POS")
            .padding((2, 2, 1, 1))
            .content(
                LinearLayout::vertical()
                    .child(Button::new_raw("Record Store Sales  ", record_sales))
                    .child(Button::new_raw("View Sales          ", view_sales))
                    .child(Button::new_raw("Manage Store Items  ", manage_items))
                    .child(Button::new_raw("Exit                ", exit_app)),
            ),

    );

    init_hotkeys;


    siv.run();

}


fn open_file(mut last_number: &mut String, mut number_items: &mut String) -> fs::File{
    let mut file: fs::File;
    let mut buffer = [0;4];
    if Path::new("inventory.dat").exists() {
        file = match fs::File::open("inventory.dat") {
            Err(why) => panic!("couldn't open: {}", why.description()),
            Ok(file) => file,
        };
        file.read(&mut buffer[..]).unwrap();
        *number_items = str::from_utf8(&buffer).unwrap().to_string();
        buffer = [0;4];
        file.read(&mut buffer[..]).unwrap();
        *last_number = str::from_utf8(&buffer).unwrap().to_string();
    }
    else {
        file = match fs::File::create("inventory.dat") {
            Err(why) => panic!("Error creating file!: {}", why.description()),
            Ok(file) => file,
        };
        let mut write_buffer = [0;4];

        write_buffer = "0".parse::<u32>().unwrap().to_ne_bytes();
        match file.write_all(&write_buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };
        write_buffer = "0".parse::<u32>().unwrap().to_ne_bytes();
        match file.write_all(&write_buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };
        fs::copy("inventory.dat", "inventory.bak");
    }

    return file;

}

fn get_inv_header(mut last_number: &mut String, mut number_items: &mut String) -> bool {
    let mut file: fs::File;
    let mut buffer = [0;4];
    //TODO format last_number and number_items to specific bytes to read and write from
    if Path::new("inventory.dat").exists() {
        file = match fs::File::open("inventory.bak") {
            Err(why) => panic!("couldn't open: {}", why.description()),
            Ok(file) => file,
        };
        file.read(&mut buffer[..]).unwrap();
        *last_number = u32::from_ne_bytes(buffer).to_string();
        file.read(&mut buffer[..]).unwrap();
        *number_items = u32::from_ne_bytes(buffer).to_string();

    }
    else {
        println!("No Inventory file found!");
        return false;
    }
    return true;
}

fn write_inv_header(mut last_number: &mut String, mut number_items: &mut String) {
    let mut file: fs::File;

    if Path::new("inventory.dat").exists() {
        file = match fs::OpenOptions::new().write(true).open("inventory.bak") {
            Err(why) => panic!("couldn't open: {}", why.description()),
            Ok(file) => file,
        };
        let mut write_buffer = last_number.clone().parse::<u32>().unwrap().to_ne_bytes();
        match file.write_all(&write_buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };
        write_buffer = number_items.clone().parse::<u32>().unwrap().to_ne_bytes();
        match file.write_all(&write_buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };

    }
    else {
        file = match fs::File::create("inventory.dat") {
            Err(why) => panic!("Error creating file!: {}", why.description()),
            Ok(file) => file,
        };
        let mut write_buffer = last_number.clone().parse::<u32>().unwrap().to_ne_bytes();
        match file.write_all(&write_buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };
        write_buffer = number_items.clone().parse::<u32>().unwrap().to_ne_bytes();
        match file.write_all(&write_buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };
    }
}

fn io_inv_body(mut file: &mut fs::File, mut id: &mut String, mut product_name: &mut String, mut product_price: &mut String, io: bool) {
    let mut last_number = String::new();
    let mut number_items = String::new();
    let mut id_buffer = [0;4];
    let mut string_bytes_buffer = [0;4];
    let mut name_buffer = vec![0u8; product_name.len()];
    let mut price_buffer = [0;8];
    get_inv_header(&mut last_number, &mut number_items);
    if Path::new("inventory.dat").exists() {
        if io == true {
            id_buffer = id.clone().parse::<u32>().unwrap().to_ne_bytes();
            string_bytes_buffer = (product_name.len() as u32).to_ne_bytes();
            name_buffer = product_name.as_bytes().to_vec();
            price_buffer = product_price.clone().parse::<f64>().unwrap().to_ne_bytes();
            match file.write_all(&id_buffer) {
                Err(why) => panic!("couldn't write: {}", why.description()),
                Ok(_) => println!("successfully wrote"),
            };
            match file.write_all(&string_bytes_buffer) {
                Err(why) => panic!("couldn't write: {}", why.description()),
                Ok(_) => println!("successfully wrote"),
            };
            match file.write_all(&name_buffer) {
                Err(why) => panic!("couldn't write: {}", why.description()),
                Ok(_) => println!("successfully wrote"),
            };
            match file.write_all(&price_buffer) {
                Err(why) => panic!("couldn't write: {}", why.description()),
                Ok(_) => println!("successfully wrote"),
            };

        }
        else {
            file.read(&mut id_buffer[..]).unwrap();
            file.read(&mut string_bytes_buffer[..]).unwrap();
            name_buffer = vec![0u8; u32::from_ne_bytes(string_bytes_buffer) as usize];
            file.read_exact(&mut name_buffer).unwrap();
            file.read(&mut price_buffer[..]).unwrap();
            *id = u32::from_ne_bytes(id_buffer).to_string();
            *product_name = String::from_utf8(name_buffer).unwrap();
            *product_price = f64::from_ne_bytes(price_buffer).to_string();
        }
    }
    else {
        println!("No inventory file found!");
    }
}

fn delete_id_inv_file(id: String) {
    let mut number_items = String::new();
    let mut last_number = String::new();
    let mut buffer = [0;4];
    let mut string_buffer: Vec<u8> = Vec::new();
    let mut price_buffer = [0;8];
    let mut file_buffer: Vec<u8> = Vec::new();
    let mut file = OpenOptions::new().read(true).write(true).open("inventory.bak").unwrap();
    let pos = search_id_inv_file(id) - 4;
    get_inv_header(&mut last_number, &mut number_items);

    file.read(&mut buffer[..]);
    file_buffer.extend_from_slice(&buffer);
    buffer = [0;4];
    file.read(&mut buffer[..]);
    file_buffer.extend_from_slice(&buffer);

    for i in 0..number_items.parse::<i32>().unwrap(){
        buffer = [0;4];
        price_buffer = [0;8];
        if file.seek(SeekFrom::Current(0)).unwrap() == pos as u64 {
            file.read(&mut buffer[..]);
            buffer = [0;4];
            file.read(&mut buffer[..]);
            string_buffer = vec![0u8; u32::from_ne_bytes(buffer) as usize];
            file.read_exact(&mut string_buffer).unwrap();
            file.read(&mut price_buffer[..]);
            continue;
        }
        file.read(&mut buffer[..]);
        file_buffer.extend_from_slice(&buffer);
        buffer = [0;4];
        file.read(&mut buffer[..]);
        string_buffer = vec![0u8; u32::from_ne_bytes(buffer) as usize];
        file_buffer.extend_from_slice(&buffer);
        file.read_exact(&mut string_buffer).unwrap();
        file_buffer.extend_from_slice(&string_buffer);
        file.read(&mut price_buffer[..]);
        file_buffer.extend_from_slice(&price_buffer);
    }

    file.seek(SeekFrom::Start(0));

    match file.write_all(&file_buffer) {
        Err(why) => panic!("couldn't write: {}", why.description()),
        Ok(_) => println!("successfully wrote"),
    };


}

fn search_id_inv_file(id: String) -> isize {
    let mut number_items = String::new();
    let mut last_number = String::new();
    let mut buffer = [0;4];
    get_inv_header(&mut last_number, &mut number_items);
    let mut file = fs::OpenOptions::new().read(true).open("inventory.bak").unwrap();
    file.seek(SeekFrom::Start(8));
    for i in 0..number_items.parse::<i32>().unwrap() {
        buffer = [0;4];
        file.read(&mut buffer[..]);
        if u32::from_ne_bytes(buffer).to_string() == id {
            println!("\npos: {}", file.seek(SeekFrom::Current(0)).unwrap());
            return file.seek(SeekFrom::Current(0)).unwrap() as isize;
        }
        buffer = [0;4];
        file.read(&mut buffer[..]);
        file.seek(SeekFrom::Current(8 + u32::from_ne_bytes(buffer) as i64));
    }

    return -1
}

fn init_hotkeys(siv: &mut Cursive) {
    siv.clear_global_callbacks(Key::Esc);
    siv.add_global_callback(Key::Esc, exit_app);
}

fn exit_app(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::text(format!("Are you sure?"))
            .title("Exit Application")
            .button("Yes", |s| {
                s.quit()
            })
            .button("No", |s| {s.pop_layer();}),
    );
}

fn get_last_sales_number(last_number: &mut u32, year: String, month: String, day: String){
    let mut file = OpenOptions::new().read(true).open("sales.dat").unwrap();
    let mut buffer = [0;4];
    let mut string_buffer = Vec::new();
    let mut number_sales = 0;
    let mut number_items = 0;
    file.read(&mut buffer[..]);
    number_sales = u32::from_ne_bytes(buffer);
    for i in 0..number_sales {
        file.read(&mut buffer[..]);
        string_buffer = vec!(0u8; u32::from_ne_bytes(buffer) as usize);
        file.read_exact(&mut string_buffer);
        let control_number = String::from_utf8(string_buffer).unwrap();
        let file_month: String = control_number.graphemes(true).skip(0).take(2).collect::<String>();
        let file_day: String = control_number.graphemes(true).skip(2).take(2).collect::<String>();
        let file_year: String = control_number.graphemes(true).skip(4).take(4).collect::<String>();
        if format!("{:0>2}", month).trim() == file_month.trim() &&
            format!("{:0>2}", day).trim() == file_day.trim() &&
            format!("{}", year).trim() == file_year.trim() {
            *last_number += 1;
        }
        file.read(&mut buffer[..]);
        number_items = u32::from_ne_bytes(buffer);
        file.seek(SeekFrom::Current(((8 * number_items) + 24) as i64));
    }
}

fn search_control_number(control_number: String) -> isize {
    let mut file = OpenOptions::new().read(true).open("sales.dat").unwrap();
    let mut buffer = [0;4];
    let mut string_buffer = Vec::new();
    let mut number_sales = 0;
    let mut number_items = 0;
    let month: String = control_number.chars().take(2).collect();
    let day: String = control_number.chars().skip(2).take(2).collect();
    let year: String = control_number.chars().skip(4).take(4).collect();
    let number: String = control_number.chars().skip(8).take(4).collect();

    file.read(&mut buffer[..]);
    number_sales = u32::from_ne_bytes(buffer);
    for i in 0..number_sales {
        file.read(&mut buffer[..]);
        string_buffer = vec!(0u8; u32::from_ne_bytes(buffer) as usize);
        file.read_exact(&mut string_buffer);
        let file_control_number = String::from_utf8(string_buffer).unwrap();
        let file_month: String = file_control_number.chars().take(2).collect();
        let file_day: String = file_control_number.chars().skip(2).take(2).collect();
        let file_year: String = file_control_number.chars().skip(4).take(4).collect();
        let file_number: String = file_control_number.chars().skip(8).take(4).collect();
        if year == file_year && month == file_month && day == file_day && number == file_number{
            return (file.seek(SeekFrom::Current(0))).unwrap() as isize;
        }
        file.read(&mut buffer[..]);
        number_items = u32::from_ne_bytes(buffer);
        file.seek(SeekFrom::Current(((8 * number_items) + 24)as i64) );
    }

    return -1;
}

fn get_daily_sales(siv: &mut Cursive, month: String, day: String, year: String) {
    let mut file = OpenOptions::new().read(true).open("sales.dat").unwrap();
    let mut pos = 0;
    let mut last_number = 0;
    let mut buffer = [0;4];
    let mut float_buffer = [0;8];
    let mut total = 0.0;
    let title = format!("Daily Sales for {:0>2}/{:0>2}/{}", month, day, year);
    get_last_sales_number(&mut last_number, year.clone(), month.clone(), day.clone());
    if last_number < 1 {
        siv.add_layer(Dialog::info("Control Number not found!"));
        return
    }
    siv.add_layer(
        Dialog::new()
            .title(title)
            .content(
                LinearLayout::vertical()
                    .child(LinearLayout::horizontal()
                        .child(TextView::new("Control Number")
                            .fixed_width(40))
                        .child(TextView::new("Number of Items")
                            .fixed_width(20))
                        .child(TextView::new("Total Sales")
                            .fixed_width(20)))
                    .child(BoxView::new(SizeConstraint::Fixed(90),
                                        SizeConstraint::Free,
                                        Panel::new(LinearLayout::vertical()
                                            //TODO Table Format, no child should be present upon final version
                                            //TODO children added via struct implementation
                                            .with_id("view_daily_sales_table"))
                                            .scrollable()))
                    .child(TextView::empty()
                        .with_id("daily_sales_total")
                        .fixed_width(20))
            )
            .dismiss_button("Back")
    );
    for i in 0..last_number{
        pos = search_control_number(format!("{:0>2}{:0>2}{}{:0>4}", month, day, year, i+1));
        let control_number = format!("{:0>2}{:0>2}{}{:0>4}", month, day, year, i+1);
        file.seek(SeekFrom::Start(pos as u64));
        file.read(&mut buffer[..]);
        let number_items = u32::from_ne_bytes(buffer) as i64;
        file.seek(SeekFrom::Current(8 * number_items));
        file.read(&mut float_buffer[..]);
        let mut date_total = f64::from_ne_bytes(float_buffer).to_string();
        date_total = format!("{:.2}", date_total.parse::<f64>().unwrap());
        total += date_total.parse::<f64>().unwrap();
        file.seek(SeekFrom::Current(16));
        siv
            .call_on_id("view_daily_sales_table", |view: &mut LinearLayout| {
                view.add_child(LinearLayout::horizontal()
                    .child(TextView::new(control_number)
                        .with_id(&("daily_control_".to_string() + &i.to_string()))
                        .fixed_width(40))
                    .child(TextView::new(number_items.to_string())
                        .with_id(&("daily_number_items_".to_string() + &i.to_string()))
                        .fixed_width(20))
                    .child(TextView::new(date_total)
                        .with_id(&("daily_total_".to_string() + &i.to_string()))
                        .fixed_width(20)))
            });
    }
    siv
        .call_on_id("daily_sales_total", |view: &mut TextView| {
            view.set_content(format!("Total: {:.2}", total))
        }).unwrap()
}

fn display_sales_details(siv: &mut Cursive, control_number: String) {
    let mut file = OpenOptions::new().read(true).open("sales.dat").unwrap();
    let pos = search_control_number(control_number.clone());
    let mut buffer = [0;4];
    let mut float_buffer = [0;8];
    let mut number_items = 0;
    let title = format!("{} Sale Details", control_number.clone());
    if pos < 0 {
        siv.add_layer(Dialog::info("Control Number not found!"));
        return;
    }
    siv.add_layer(
        Dialog::new()
            .title(title)
            .content(
                LinearLayout::vertical()
                    .child(LinearLayout::horizontal()
                        .child(TextView::new("Item No.")
                            .fixed_width(10))
                        .child(TextView::new("Product ID")
                            .fixed_width(40))
                        .child(TextView::new("Product Name")
                            .fixed_width(40))
                        .child(TextView::new("Quantity")
                            .fixed_width(10))
                        .child(TextView::new("Price")
                            .fixed_width(10)))
                    .child(BoxView::new(SizeConstraint::Fixed(120),
                                        SizeConstraint::Fixed(10),
                                        Panel::new(LinearLayout::vertical()
                                            //TODO Table Format, no child should be present upon final version
                                            //TODO children added via struct implementation
                                            .with_id("sales_details_table"))
                                            .fixed_width(112)
                                            .scrollable()))
                    .child(LinearLayout::horizontal()
                        .child(BoxView::new(SizeConstraint::Fixed(70),
                                            SizeConstraint::Free,
                                            Panel::new(ListView::new()
                                                .child("Tendered Amount:", TextView::empty()
                                                    .with_id("tendered_amount_detail")
                                                    .fixed_width(20))
                                                .child("Change:", TextView::empty()
                                                    .with_id("change_detail")
                                                    .fixed_width(20)))))
                        .child(LinearLayout::vertical()
                            .child(TextView::new("Total:"))
                            .child(BoxView::new(SizeConstraint::Fixed(42),
                                                SizeConstraint::Fixed(5),
                                                Panel::new(TextView::new("0.00")
                                                    .center()
                                                    .with_id("sales_detail_total")))))),
            )
            .dismiss_button("Back")
    );
    file.seek(SeekFrom::Start(pos as u64));
    file.read(&mut buffer[..]);
    number_items = u32::from_ne_bytes(buffer);
    for index in 0..number_items {
        file.read(&mut buffer[..]);
        let mut product_id = u32::from_ne_bytes(buffer).to_string();
        file.read(&mut buffer[..]);
        let qty = u32::from_ne_bytes(buffer).to_string();
        let mut product_name = String::new();
        let mut product_price = String::new();
        get_inv_item_details(siv, &mut product_id, &mut product_name, &mut product_price);
        siv
            .call_on_id("sales_details_table", |view: &mut LinearLayout| {
                let id = product_id.clone();
                let name = product_name.clone();
                let mut price = product_price.clone();
                price = format!("{:.2}", price.parse::<f64>().unwrap() * qty.parse::<f64>().unwrap());
                view.add_child(LinearLayout::horizontal()
                    .child(TextView::new(&(index+1).to_string())
                        .with_id(&("sales_item_no_".to_string() + &(index+1).to_string()))
                        .fixed_width(10))
                    .child(TextView::new(&id)
                        .with_id(&("sales_id_".to_string() + &(index+1).to_string()))
                        .fixed_width(40))
                    .child(TextView::new(&name)
                        .with_id(&("sales_name_".to_string() + &(index+1).to_string()))
                        .fixed_width(40))
                    .child(TextView::new(qty)
                        .with_id(&("sales_qty_".to_string() + &(index+1).to_string()))
                        .fixed_width(10))
                    .child(TextView::new(&price)
                        .with_id(&("sales_price_".to_string() + &(index+1).to_string()))
                        .fixed_width(10))
                    .with_id(&("sales_item_".to_string() + &(index+1).to_string())))
            });
    }
    file.read(&mut float_buffer[..]);
    let mut total = f64::from_ne_bytes(float_buffer).to_string();
    file.read(&mut float_buffer[..]);
    let mut tendered = f64::from_ne_bytes(float_buffer).to_string();
    file.read(&mut float_buffer[..]);
    let mut change = f64::from_ne_bytes(float_buffer).to_string();
    total = format!("{:.2}", total.parse::<f64>().unwrap());
    tendered = format!("{:.2}", tendered.parse::<f64>().unwrap());
    change = format!("{:.2}", change.parse::<f64>().unwrap());
    siv
        .call_on_id("tendered_amount_detail", |view: &mut TextView| {
            view.set_content(tendered);
        });
    siv
        .call_on_id("change_detail", |view: &mut TextView| {
            view.set_content(change);
        });
    siv
        .call_on_id("sales_detail_total", |view: &mut TextView| {
            view.set_content(total);
        });
}

fn id_corresponds_name(siv: &mut Cursive) -> bool {
    let id = siv
        .call_on_id("search_product_id", |view: &mut EditView| {
            view.get_content()
        })
        .unwrap().to_string();
    let name = siv
        .call_on_id("search_product_name", |view: &mut EditView| {
            view.get_content()
        })
        .unwrap().to_string();
    if id.is_empty() || name.is_empty() {
        return false;
    }
    let pos = search_id_inv_file(id.to_string()) - 4;
    if pos < 0 {
        return false;
    }
    let mut buffer = [0;4];
    let mut name_buffer = Vec::new();
    let mut inv_id = String::new();
    let mut inv_name = String::new();
    let mut file = OpenOptions::new().read(true).open("inventory.bak").unwrap();
    file.seek(SeekFrom::Start(pos as u64));
    file.read(&mut buffer[..]);
    inv_id = u32::from_ne_bytes(buffer).to_string();
    buffer = [0;4];
    file.read(&mut buffer[..]);
    name_buffer = vec![0u8; u32::from_ne_bytes(buffer) as usize];
    file.read_exact(&mut name_buffer);
    inv_name = String::from_utf8(name_buffer).unwrap();
    if id == inv_id && name == inv_name {
        return true;
    }

    return false;
}

fn get_inv_item_details(siv: &mut Cursive, product_id: &mut String, product_name: &mut String, product_price: &mut String) -> bool{
    let pos = search_id_inv_file(product_id.clone()) - 4;
    if pos < 0 {
        siv.add_layer(Dialog::info("Product ID not found!"));
        return false;
    }
    let mut buffer = [0;4];
    let mut name_buffer = Vec::new();
    let mut price_buffer = [0;8];
    let mut file = OpenOptions::new().read(true).open("inventory.bak").unwrap();
    file.seek(SeekFrom::Start(pos as u64));
    file.read(&mut buffer[..]);
    *product_id = u32::from_ne_bytes(buffer).to_string();
    file.read(&mut buffer[..]);
    name_buffer = vec![0u8; u32::from_ne_bytes(buffer) as usize];
    file.read_exact(&mut name_buffer).unwrap();
    *product_name = String::from_utf8(name_buffer).unwrap();
    file.read(&mut price_buffer[..]);
    *product_price = f64::from_ne_bytes(price_buffer).to_string();
    return true;
}

fn input_sales_record(siv: &mut Cursive) {
    let mut file: fs::File;
    let mut number_sales: u32 = 0;
    let mut number_items: u32 = 0;
    let mut amount_tendered: f64 = 0.0;
    let mut change: f64 = 0.0;
    let mut buffer = [0;4];
    let mut float_buffer = [0;8];
    let mut string_buffer = Vec::new();
    let now = Local::now();
    let (era, year) = now.year_ce();
    if Path::new("sales.dat").exists() {
        file = OpenOptions::new().read(true).write(true).open("sales.dat").unwrap();
        file.read(&mut buffer[..]);
        number_sales = u32::from_ne_bytes(buffer);
        number_sales += 1;
        file.seek(SeekFrom::Start(0));
        buffer = number_sales.to_ne_bytes();
        match file.write_all(&buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };

        file = OpenOptions::new().append(true).open("sales.dat").unwrap();
    }
    else {
        file = match fs::File::create("sales.dat") {
            Err(why) => panic!("Error creating file!: {}", why.description()),
            Ok(file) => file,
        };
        number_sales = 1;
        buffer = number_sales.to_ne_bytes();
        match file.write_all(&buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };
    }
    let mut sales_number = 0;
    get_last_sales_number(&mut sales_number, year.to_string(), now.month().to_string(), now.day().to_string());
    sales_number += 1;
    let control_number = format!("{:0>2}{:0>2}{}{:0>4}", now.month(), now.day(), year, sales_number);
    buffer = (control_number.len() as u32).to_ne_bytes();
    match file.write_all(&buffer) {
        Err(why) => panic!("couldn't write: {}", why.description()),
        Ok(_) => println!("successfully wrote"),
    };
    string_buffer = vec![0u8; control_number.len() as usize];
    string_buffer = control_number.as_bytes().to_vec();
    match file.write_all(&string_buffer) {
        Err(why) => panic!("couldn't write: {}", why.description()),
        Ok(_) => println!("successfully wrote"),
    };
    buffer = (get_number_items(siv) as u32).to_ne_bytes();
    match file.write_all(&buffer) {
        Err(why) => panic!("couldn't write: {}", why.description()),
        Ok(_) => println!("successfully wrote"),
    };
    for index in 0..get_last_number_used(siv) + 1 {
        if !siv
            .call_on_id(&("sales_item_".to_string() + &(index+1).to_string()), |view: &mut LinearLayout| {
                view.is_empty()
            }).is_none() {
            let id = siv
                .call_on_id(&("sales_id_".to_string() + &(index+1).to_string()), |view: &mut TextView| {
                    view.get_content()
                }).unwrap().source().to_string();
            buffer = id.parse::<u32>().unwrap().to_ne_bytes();
            match file.write_all(&buffer) {
                Err(why) => panic!("couldn't write: {}", why.description()),
                Ok(_) => println!("successfully wrote"),
            };
            let qty = siv
                .call_on_id(&("sales_qty_".to_string() + &(index+1).to_string()), |view: &mut TextView| {
                    view.get_content()
                }).unwrap().source().to_string();
            buffer = qty.parse::<u32>().unwrap().to_ne_bytes();
            match file.write_all(&buffer) {
                Err(why) => panic!("couldn't write: {}", why.description()),
                Ok(_) => println!("successfully wrote"),
            };
        }
    }
    let total = siv
        .call_on_id("customer_sales_total", |view: &mut TextView| {
            view.get_content()
        }).unwrap().source().to_string();
    float_buffer = total.parse::<f64>().unwrap().to_ne_bytes();
    match file.write_all(&float_buffer) {
        Err(why) => panic!("couldn't write: {}", why.description()),
        Ok(_) => println!("successfully wrote"),
    };
    let amount_tendered = siv
        .call_on_id("amount_tendered", |view: &mut EditView| {
            view.get_content()
        }).unwrap().to_string();
    float_buffer = amount_tendered.parse::<f64>().unwrap().to_ne_bytes();
    match file.write_all(&float_buffer) {
        Err(why) => panic!("couldn't write: {}", why.description()),
        Ok(_) => println!("successfully wrote"),
    };
    let change = siv
        .call_on_id("sales_change", |view: &mut TextView| {
            view.get_content()
        }).unwrap().source().to_string();
    float_buffer = change.parse::<f64>().unwrap().to_ne_bytes();
    match file.write_all(&float_buffer) {
        Err(why) => panic!("couldn't write: {}", why.description()),
        Ok(_) => println!("successfully wrote"),
    };
}

fn get_number_sales() -> u32{
    let mut number_sales = 0;
    if Path::new("sales.dat").exists() {
        let mut file = OpenOptions::new().read(true).open("sales.dat").unwrap();
        let mut buffer = [0; 4];
        file.read(&mut buffer[..]);
        number_sales = u32::from_ne_bytes(buffer);
    }
    return number_sales
}

fn get_last_number_used(siv: &mut Cursive) -> isize{
    let mut number_items = get_number_items(siv);
    let mut index = 0;
    while number_items > 0 {
        if siv
            .call_on_id(&("sales_price_".to_string() + &(index+1).to_string()), |view: &mut TextView| {
                view.get_content()
            }).is_none() {
            index += 1;
            continue;
        }
        else {
            number_items -= 1;
        }
        index += 1;
    }
    return index
}

fn enter_item(siv: &mut Cursive, qty: &str) {
    let mut number_items= 0;
    let mut product_id = siv
        .call_on_id("search_product_id", |view: &mut EditView| {
            view.get_content()
        })
        .unwrap().to_string();
    let mut product_name = siv
        .call_on_id("search_product_name", |view: &mut EditView| {
            view.get_content()
        })
        .unwrap().to_string();
    let mut product_price = siv
        .call_on_id("search_product_price", |view: &mut TextView| {
            view.get_content()
        })
        .unwrap().source().to_string();
    let mut total = siv
        .call_on_id("customer_sales_total", |view: &mut TextView| {
            view.get_content()
        }).unwrap().source().to_string();
    if in_sales_table(siv, product_id.clone()) {
        siv.add_layer(Dialog::info("Product ID already punched!"));
        empty_search(siv);
        return
    }
    if qty.parse::<usize>().is_err() || qty.parse::<usize>().unwrap() == 0 {
        siv.add_layer(Dialog::info("Invalid quantity!"));
        siv
            .call_on_id("search_product_quantity", |view: &mut EditView| {
                view.set_content("");
            });
        siv.focus_id("search_product_quantity");
        return;
    }
    number_items = get_number_items(siv);
    let last_number = get_last_number_used(siv);
    for index in 0..last_number + 1{
        if siv
            .call_on_id(&("sales_item_".to_string() + &(index+1).to_string()), |view: &mut LinearLayout| {
                view.is_empty()
            }).is_none() && last_number < index + 1 {
            siv
                .call_on_id("customer_sales_table", |view: &mut LinearLayout| {
                    let id = product_id.clone();
                    let name = product_name.clone();
                    let mut price = product_price.clone();
                    price = format!("{:.2}", price.parse::<f64>().unwrap() * qty.parse::<f64>().unwrap());
                    view.add_child(LinearLayout::horizontal()
                        .child(TextView::new(&(index+1).to_string())
                            .with_id(&("sales_item_no_".to_string() + &(index+1).to_string()))
                            .fixed_width(10))
                        .child(TextView::new(&id)
                            .with_id(&("sales_id_".to_string() + &(index+1).to_string()))
                            .fixed_width(40))
                        .child(TextView::new(&name)
                            .with_id(&("sales_name_".to_string() + &(index+1).to_string()))
                            .fixed_width(40))
                        .child(TextView::new(qty)
                            .with_id(&("sales_qty_".to_string() + &(index+1).to_string()))
                            .fixed_width(10))
                        .child(TextView::new(&price)
                            .with_id(&("sales_price_".to_string() + &(index+1).to_string()))
                            .fixed_width(10))
                        .with_id(&("sales_item_".to_string() + &(index+1).to_string())))
                });
        }
    }
    update_sales_total(siv);
    update_item_no(siv);
    empty_search(siv);
}

fn empty_search(siv: &mut Cursive) {
    siv
        .call_on_id("search_product_name", |view: &mut EditView| {
            view.set_content("");
        });
    siv
        .call_on_id("search_product_price", |view: &mut TextView| {
            view.set_content("");
        });
    siv
        .call_on_id("search_product_quantity", |view: &mut EditView| {
            view.set_content("");
        });
    siv
        .call_on_id("search_product_id", |view: &mut EditView| {
            view.set_content("");
        });
    siv.focus_id("search_product_id");
}

fn get_number_items(siv: &mut Cursive) -> isize {
    let mut index = 0;
    index = siv
        .call_on_id("customer_sales_table", |view: &mut LinearLayout| {
            view.len()
        }).unwrap();
    return index as isize
}

fn in_sales_table(siv: &mut Cursive, id: String) -> bool {
    let number_items = get_number_items(siv);
    for index in 0..number_items {
        if siv
            .call_on_id(&("sales_id_".to_string() + &(index+1).to_string()), |view: &mut TextView| {
                view.get_content()
            }).is_none() {continue};
        if siv
            .call_on_id(&("sales_id_".to_string() + &(index+1).to_string()), |view: &mut TextView| {
                view.get_content()
            }).unwrap().source() == id {
            return true
        }
    }
    return false
}

fn search_item(siv: &mut Cursive, name: &str) {
    let mut id = siv
        .call_on_id("search_product_id", |view: &mut EditView| {
            view.get_content()
        })
        .unwrap().to_string();
    let mut name = siv
        .call_on_id("search_product_name", |view: &mut EditView| {
            view.get_content()
        })
        .unwrap().to_string();
    let mut price = String::new();

    if id.parse::<usize>().is_err() {
        siv.add_layer(Dialog::info("Invalid Product ID!"));
        empty_search(siv);
        return;
    }

    if id_corresponds_name(siv) {
        siv.focus_id("search_product_quantity");
    }
    else {
        if !get_inv_item_details(siv,&mut id, &mut name, &mut price) {
            siv
                .call_on_id("search_product_id", |view: &mut EditView| {
                    view.set_content("");
                });
            return;
        }
        price = format!("{:.2}", price.parse::<f64>().unwrap());
        siv
            .call_on_id("search_product_name", |view: &mut EditView| {
                view.set_content(name);
            });
        siv
            .call_on_id("search_product_price", |view: &mut TextView| {
                view.set_content(price);
            });
        siv
            .call_on_id("search_product_quantity", |view: &mut EditView| {
                view.set_content("");
            });

        siv.focus_id("search_product_quantity");
    }
}

fn update_sales_total(siv: &mut Cursive) {
    let mut number_items = get_number_items(siv);
    let mut total = "0.0".to_string();
    let mut index = 0;
    while number_items > 0 {
        if siv
            .call_on_id(&("sales_price_".to_string() + &(index+1).to_string()), |view: &mut TextView| {
                view.get_content()
            }).is_none() {
            index += 1;
            continue;
        }
        else {
            total = (siv
                .call_on_id(&("sales_price_".to_string() + &(index + 1).to_string()), |view: &mut TextView| {
                    view.get_content()
                }).unwrap().source().to_string().parse::<f64>().unwrap() + total.parse::<f64>().unwrap()).to_string();
            number_items -= 1;
        }
        index += 1;
    }
    total = format!("{:.2}", total.parse::<f64>().unwrap());
    siv
        .call_on_id("customer_sales_total", |view: &mut TextView| {
            view.set_content(total);
        });
}

fn update_item_no(siv: &mut Cursive) {
    let mut number_items = get_number_items(siv);
    let mut number = 1;
    let mut i = 0;
    while number_items > 0 {
        if siv
            .call_on_id(&("sales_item_no_".to_string() + &(i+1).to_string()), |view: &mut TextView| {
                view.get_content()
            }).is_none() {
            i += 1;
            continue;
        }
        else {
            siv
                .call_on_id(&("sales_item_no_".to_string() + &(i + 1).to_string()), |view: &mut TextView| {
                    view.set_content((number).to_string())
                });
            number += 1;
            number_items -= 1;
        }
        i += 1;
    }
}

fn compute_change(siv: &mut Cursive, amount: &str, size: usize) {
    let total = siv
        .call_on_id("customer_sales_total", |view: &mut TextView| {
            view.get_content()
        }).unwrap().source().to_string();
    let mut tendered = (amount.clone()).to_string();
    let mut change = String::new();
    if !tendered.parse::<f64>().is_err() {
        tendered = format!("{:.2}", tendered.parse::<f64>().unwrap());
        change = (tendered.parse::<f64>().unwrap() - total.parse::<f64>().unwrap()).to_string();
        change = format!("{:.2}", change.parse::<f64>().unwrap());
        if change.parse::<f64>().unwrap() > 0.0 {
            siv
                .call_on_id("sales_change", |view: &mut TextView| {
                    view.set_content(change)
                });
        }
    }
    else {
        siv
            .call_on_id("sales_change", |view: &mut TextView| {
                view.set_content("0.00".to_string())
            });
    }
}

fn tender_amount(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Enter Amount")
            .content( ListView::new()
                .child("Tendered", EditView::new()
                    .on_edit(compute_change)
                    .with_id("amount_tendered")
                    .fixed_width(20))
                .child("Change", TextView::new("0.00")
                    .with_id("sales_change")
                    .fixed_width(20)))
            .button("Confirm", |s| {
                let total = s
                    .call_on_id("customer_sales_total", |view: &mut TextView| {
                        view.get_content()
                    }).unwrap().source().to_string();
                let tendered = s
                    .call_on_id("amount_tendered", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap().to_string();
                if tendered.parse::<f64>().is_err() {
                    s.add_layer(Dialog::info("Invalid amount!"));
                }
                else if tendered.parse::<f64>().unwrap() < total.parse::<f64>().unwrap() {
                    s.add_layer(Dialog::info("Insufficient amount!"));
                }
                else {
                    input_sales_record(s);
                    s.pop_layer();
                    s.pop_layer();
                    record_sales(s);
                }
            })
            .dismiss_button("Back")
    );
}

fn dummy_callback(siv: &mut Cursive, name: &str) {}

fn record_sales(siv: &mut Cursive) {
    let mut inv_items = String::new();
    let mut inv_last_number = String::new();
    if !get_inv_header(&mut inv_last_number, &mut inv_items) || inv_items.is_empty() || inv_items == "0".to_string() {
        siv.add_layer(Dialog::info("Inventory is empty! Please add items to inventory first."));
        return
    }
    siv.add_layer(
        Dialog::new()
            .content(
                LinearLayout::vertical()
                    .child(LinearLayout::horizontal()
                        .child(TextView::new("Item No.")
                            .fixed_width(10))
                        .child(TextView::new("Product ID")
                            .fixed_width(40))
                        .child(TextView::new("Product Name")
                            .fixed_width(40))
                        .child(TextView::new("Quantity")
                            .fixed_width(10))
                        .child(TextView::new("Price")
                            .fixed_width(10)))
                    .child(BoxView::new(SizeConstraint::Fixed(120),
                                        SizeConstraint::Fixed(10),
                                        Panel::new(LinearLayout::vertical()
                                            //TODO Table Format, no child should be present upon final version
                                            //TODO children added via struct implementation
                                            .with_id("customer_sales_table"))
                                            .fixed_width(112)
                                            .scrollable()))
                    .child(LinearLayout::horizontal()
                        .child(BoxView::new(SizeConstraint::Fixed(70),
                                            SizeConstraint::Free,
                                            Panel::new(ListView::new()
                                                .child("Product ID", EditView::new()
                                                    .on_submit(search_item)
                                                    .with_id("search_product_id")
                                                    .fixed_width(20))
                                                .child("Product Name", EditView::new()
                                                    .on_submit( dummy_callback)
                                                    .with_id("search_product_name")
                                                    .fixed_width(20))
                                                .child("Quantity", EditView::new()
                                                    .on_submit( enter_item)
                                                    .with_id("search_product_quantity")
                                                    .fixed_width(20))
                                                .child("Price", TextView::empty()
                                                    .with_id("search_product_price"))
                                                .with_id("record_sales_table"))))
                        .child(LinearLayout::vertical()
                            .child(TextView::new("Total:"))
                            .child(BoxView::new(SizeConstraint::Fixed(42),
                                                SizeConstraint::Fixed(5),
                                                Panel::new(TextView::new("0.00")
                                                    .center()
                                                    .with_id("customer_sales_total")))))),
            )
            .button("Tender Amount", tender_amount)
            .button("Edit Item",  |s| {edit_remove_popup(s, false, false)})
            .button("Remove Item", |s| {edit_remove_popup(s, true, false)})
            .button("Clear", |s| {
                s.pop_layer();
                record_sales(s);
            })
            .dismiss_button(("Esc|Back")),
    );
    siv.clear_global_callbacks(Key::Esc);
    siv.add_global_callback(Key::Esc, |s| {s.pop_layer(); s.clear_global_callbacks(Key::Esc); init_hotkeys;});
}

fn view_sales(siv: &mut Cursive) {
    if get_number_sales() < 1 {
        siv.add_layer(Dialog::info("No sales found!"));
        return
    }
    siv.add_layer(
        Dialog::new()
            .content(
                LinearLayout::vertical()
                    .child(LinearLayout::horizontal()
                        .child(TextView::new("Control Number")
                            .fixed_width(40))
                        .child(TextView::new("Number of Items")
                            .fixed_width(20))
                        .child(TextView::new("Total Sales")
                            .fixed_width(20)))
                    .child(BoxView::new(SizeConstraint::Fixed(90),
                                        SizeConstraint::Free,
                                        Panel::new(LinearLayout::vertical()
                                            //TODO Table Format, no child should be present upon final version
                                            //TODO children added via struct implementation
                                            .with_id("view_sales_table"))
                                            .scrollable()))
            )
            .button("Search", |s| {
                s.add_layer(
                    Dialog::new()
                        .title("Enter Detail(s)")
                        .content(
                            ListView::new()
                                .child("Control Number", EditView::new()
                                    .with_id("search_control_number")
                                    .fixed_width(20))
                        )
                        .button("Confirm", |s| {
                            let control_number = s
                                .call_on_id("search_control_number", |view: &mut EditView| {
                                    view.get_content()
                                }).unwrap().to_string();
                            if control_number.chars().count() > 12 && control_number.chars().count() < 0 {
                                s.add_layer(Dialog::info("Invalid Control Number!"));
                            }
                            else {
                                s.pop_layer();
                                display_sales_details(s, control_number);
                            }
                        })
                        .dismiss_button("Back")
                )
            })
            .button("View Daily Sales", |s| {
                s.add_layer(
                    Dialog::new()
                        .title("Enter date")
                        .content(
                            ListView::new()
                                .child("Month:", EditView::new()
                                    .with_id("search_month")
                                    .fixed_width(20))
                                .child("Day:", EditView::new()
                                    .with_id("search_day")
                                    .fixed_width(20))
                                .child("Year:", EditView::new()
                                    .with_id("search_year")
                                    .fixed_width(20))
                        )
                        .button("Confirm", |s| {
                            let month = s
                                .call_on_id("search_month", |view: &mut EditView| {
                                    view.get_content()
                                }).unwrap().to_string();
                            let day = s
                                .call_on_id("search_day", |view: &mut EditView| {
                                    view.get_content()
                                }).unwrap().to_string();
                            let year = s
                                .call_on_id("search_year", |view: &mut EditView| {
                                    view.get_content()
                                }).unwrap().to_string();
                            if month.parse::<i32>().is_err() ||
                                (month.parse::<i32>().unwrap() > 12 || month.parse::<i32>().unwrap() < 1) {
                                s.add_layer(Dialog::info("Invalid month!"));
                            }
                            else if day.parse::<i32>().is_err() ||
                                (day.parse::<i32>().unwrap() > 31 || day.parse::<i32>().unwrap() < 1) {
                                s.add_layer(Dialog::info("Invalid day!"));
                            }
                            else if year.parse::<i32>().is_err() || year.parse::<i32>().unwrap() < 0  {
                                s.add_layer(Dialog::info("Invalid year!"));
                            }
                            else {
                                s.pop_layer();
                                get_daily_sales(s, month, day, year);
                            }
                        })
                        .dismiss_button("Back")
                )
            })
            .dismiss_button("Back")
    );
    if get_number_sales() > 0 {
        init_table(siv, 2);
    }
}

fn add_item(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Enter Item Details")
            .button("Add", |s| {
                let add_item_product_name = s
                    .call_on_id("add_product_name", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                let add_item_product_price = s
                    .call_on_id("add_product_price", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                if add_item_product_name.is_empty() || add_item_product_price.is_empty() {
                    s.add_layer(Dialog::info("Field(s) cannot be empty!"));
                }
                else if (add_item_product_price.parse::<f64>()).is_err() {
                    s.add_layer(Dialog::info("Invalid price!"));
                }
                else {
                    add_prompt(s);
                }
            })
            .button("Esc|Back", |s| {s.pop_layer();})
            .content(
                ListView::new()
                    .child("Product Name", EditView::new()
                        .with_id("add_product_name")
                        .fixed_width(10))
                    .child("Product Price", EditView::new()
                        .with_id("add_product_price")
                        .fixed_width(10))
            )
    );
}

fn add_prompt(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::text(format!("Are you sure?"))
            .title(("Add Item"))
            .button("Yes", |s|{
                let mut name: String = s
                    .call_on_id("add_product_name", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap().parse().unwrap();
                let mut price: String = s
                    .call_on_id("add_product_price", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap().parse().unwrap();
                s.pop_layer();

                let mut number_of_items = String::new();
                let mut last_number = String::new();
                let index:isize = 0;
                get_inv_header(&mut last_number, &mut number_of_items);

                for index in 0..last_number.parse::<isize>().unwrap() + 1 {
                    if s
                        .call_on_id(&("manage_product_".to_string() + &(index+1).to_string()), |view: &mut LinearLayout| {
                            view.is_empty()
                        }).is_none() && last_number.parse::<isize>().unwrap() < index + 1{
                        s
                            .call_on_id("manage_product_table", |view: &mut LinearLayout| {
                                let mut file = OpenOptions::new().append(true).open("inventory.bak").unwrap();
                                let add_name = name.clone();
                                let mut add_price = price.clone();
                                add_price = format!("{:.2}", add_price.parse::<f64>().unwrap());
                                number_of_items = (number_of_items.parse::<usize>().unwrap() + 1).to_string();
                                last_number = (index+1).to_string();
                                write_inv_header(&mut last_number, &mut number_of_items);
                                io_inv_body(&mut file, &mut (index+1).to_string(), &mut name, &mut price, true);
                                view.add_child(LinearLayout::horizontal()
                                    .child(TextView::new(&(index+1).to_string())
                                        .with_id(&("manage_product_id_".to_string() + &(index+1).to_string()))
                                        .fixed_width(40))
                                    .child(TextView::new(add_name)
                                        .with_id(&("manage_product_name_".to_string() + &(index+1).to_string()))
                                        .fixed_width(40))
                                    .child(TextView::new(add_price)
                                        .with_id(&("manage_product_price_".to_string() + &(index+1).to_string()))
                                        .fixed_width(10))
                                    .with_id(&("manage_product_".to_string() + &(index+1).to_string())))
                            });
                    }
                }

                println!("\n# of items: {}\nLast number: {}", number_of_items, last_number);
                s.call_on_id("add_product_name", |view: &mut EditView| {
                    view.set_content("")
                })
                    .unwrap();
                s.call_on_id("add_product_price", |view: &mut EditView| {
                    view.set_content("")
                })
                    .unwrap();
                s.focus_id("add_product_name").unwrap();
            })
            .button("No", |s| {s.pop_layer();})
    )
}

fn search_parent(siv: &mut Cursive, id: &String, inventory: bool) -> isize {
    let mut result: isize = -1;
    let mut last_number_used = String::new();
    let mut number_of_items = String::new();
    if inventory {
        if get_inv_header(&mut last_number_used, &mut number_of_items) == false {
            return -1;
        }
    }
    else {
        number_of_items = get_number_items(siv).to_string();
    }

    for i in 0..number_of_items.parse::<usize>().unwrap() + 1 {
        let dummy_view = TextView::empty();
        let view_text : TextContentRef;
        if inventory {
            view_text = siv
                .call_on_id(&("manage_product_id_".to_string() + &(i.to_string())), |view: &mut TextView| {
                    view.get_content()
                })
                .unwrap_or(dummy_view.get_content());
        }
        else {
            view_text = siv
                .call_on_id(&("sales_id_".to_string() + &(i.to_string())), |view: &mut TextView| {
                    view.get_content()
                })
                .unwrap_or(dummy_view.get_content());
        }
        if *view_text.source() == *id {
            println!("{}", i);
            return i as isize;
        }

    }

    return result
}

fn get_item_details(siv: &mut Cursive, id: &String, mut product_name: &mut String, mut product_price: &mut String, inventory: bool) {
    let index = search_parent(siv, id, inventory);
    if index == -1 {
        return;
    }
    if inventory {
        *product_name = (*siv
            .call_on_id(&("manage_product_name_".to_string() + &index.to_string()), |view: &mut TextView| {
                view.get_content()
            })
            .unwrap().source()).to_string();
        *product_price = (*siv
            .call_on_id(&("manage_product_price_".to_string() + &index.to_string()), |view: &mut TextView| {
                view.get_content()
            })
            .unwrap().source()).to_string();
    }
    else {
        *product_name = (*siv
            .call_on_id(&("sales_name_".to_string() + &index.to_string()), |view: &mut TextView| {
                view.get_content()
            })
            .unwrap().source()).to_string();
        *product_price = (*siv
            .call_on_id(&("sales_price_".to_string() + &index.to_string()), |view: &mut TextView| {
                view.get_content()
            })
            .unwrap().source()).to_string();
    }
}

fn update_table(siv: &mut Cursive, id: String, inventory: bool) {
    let index = search_parent(siv, &id, inventory);
    if index < 0 {
        siv.add_layer(Dialog::info("Product ID not found!"));
        return;
    }
    if inventory == true {
        let mut product_name: String = (siv
            .call_on_id("edit_product_name", |view: &mut EditView| {
                view.get_content()
            })
            .unwrap()).parse().unwrap();
        let mut product_price: String = siv
            .call_on_id("edit_product_price", |view: &mut EditView| {
                view.get_content()
            })
            .unwrap().parse().unwrap();
        product_price = format!("{:.2}", product_price.parse::<f64>().unwrap());
        let name = product_name.clone();
        let price = product_price.clone();
        siv
            .call_on_id(&("manage_product_name_".to_string() + &index.to_string()), |view: &mut TextView| {
                view.set_content(product_name)
            });
        siv
            .call_on_id(&("manage_product_price_".to_string() + &index.to_string()), |view: &mut TextView| {
                view.set_content(product_price)
            });
        let pos = search_id_inv_file(id);
        let mut size_buffer = [0;4];
        let mut string_buffer = vec![0u8; name.len()];
        let mut price_buffer = [0;8];
        let mut file = OpenOptions::new().write(true).open("inventory.bak").unwrap();
        file.seek(SeekFrom::Start(pos as u64));
        size_buffer = (name.len() as u32).to_ne_bytes();
        string_buffer = name.as_bytes().to_vec();
        price_buffer = price.clone().parse::<f64>().unwrap().to_ne_bytes();
        match file.write_all(&size_buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };
        match file.write_all(&string_buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };
        match file.write_all(&price_buffer) {
            Err(why) => panic!("couldn't write: {}", why.description()),
            Ok(_) => println!("successfully wrote"),
        };

    } else {
        let qty = siv
            .call_on_id("edit_product_qty", |view: &mut EditView| {
                view.get_content()
            }).unwrap().to_string();
        let mut product_name = String::new();
        let mut product_price = String::new();
        let mut product_id = id.clone();
        get_inv_item_details(siv, &mut product_id, &mut product_name, &mut product_price);
        let price = format!("{:.2}", product_price.parse::<f64>().unwrap() * qty.parse::<f64>().unwrap());
        siv
            .call_on_id(&("sales_qty_".to_string() + &index.to_string()), |view: &mut TextView| {
                view.set_content(qty)
            });
        siv
            .call_on_id(&("sales_price_".to_string() + &index.to_string()), |view: &mut TextView| {
                view.set_content(price)
            });
        update_sales_total(siv);
    }
}

fn edit_item(siv: &mut Cursive, id: String, remove: bool, inventory: bool) {
    let mut last_number_used : String = "".to_string();
    let mut number_of_items: String = "".to_string();
    if get_inv_header(&mut last_number_used, &mut number_of_items) == false {
        println!("Error getting Inventory header!");
        return;
    }
    let mut index = search_parent(siv, &id, inventory) - 1;
    if index < 0 {
        siv.add_layer(Dialog::info("Product ID not found!"));
        return;
    }
    let mut product_name = String::new();
    let mut product_price = String::new();
    let product_id = id.clone();
    get_item_details(siv, &product_id, &mut product_name, &mut product_price, inventory);
    //TODO edit/remove item from table then pass through update_items for database
    if remove == true {
        if inventory {
            delete_id_inv_file(id.clone());
            siv
                .call_on_id("manage_product_table", |view: &mut LinearLayout| {
                    view.remove_child(index as usize);
                    number_of_items = (number_of_items.parse::<usize>().unwrap() - 1).to_string();
                    println!("Number of items: {}", number_of_items);
                });
        }
        else {
            siv
                .call_on_id("customer_sales_table", |view: &mut LinearLayout| {
                    view.remove_child(index as usize);
                });
            update_item_no(siv);
            update_sales_total(siv);
        }
    }
    else {
        siv.add_layer(
            Dialog::new()
                .title("Edit Product Details")
                .button("Edit", move |s| {
                    //TODO edit manage table
                    if inventory {
                        let edit_product_name = s
                            .call_on_id("edit_product_name", |view: &mut EditView| {
                                view.get_content()
                            })
                            .unwrap();
                        let edit_product_price = s
                            .call_on_id("edit_product_price", |view: &mut EditView| {
                                view.get_content()
                            })
                            .unwrap();
                        if edit_product_name.is_empty() || edit_product_price.is_empty() {
                            s.add_layer(Dialog::info("Field(s) cannot be empty!"));
                        }
                        else if (edit_product_price.parse::<f64>()).is_err() {
                            s.add_layer(Dialog::info("Invalid price!"));
                        }
                        else {
                            let id_edit = id.clone();
                            update_table(s, id_edit, inventory);
                            s.pop_layer();
                        }
                    }
                    else {
                        let id_edit = id.clone();
                        update_table(s, id_edit, inventory);
                        s.pop_layer();
                    }

                })
                .button("Esc|Back", |s| {s.pop_layer();})
                .content(ListView::new()
                    .with_id("edit_popup"))
        );
        if inventory {
            siv
                .call_on_id("edit_popup", |view: &mut ListView| {
                    view.add_child("Product ID", TextView::new(&(index+1).to_string())
                        .fixed_width(20));
                    view.add_child("Product Name", EditView::new()
                        .content(product_name)
                        .with_id("edit_product_name")
                        .fixed_width(20));
                    view.add_child("Product Price", EditView::new()
                        .content(product_price)
                        .with_id("edit_product_price")
                        .fixed_width(20));
                });
            write_inv_header(&mut last_number_used, &mut number_of_items);
        }
        else {
            let qty = siv
                .call_on_id(&("sales_qty_".to_string() + &(index+1).to_string()), |view: &mut TextView| {
                    view.get_content()
                }).unwrap().source().to_string();
            siv
                .call_on_id("edit_popup", |view: &mut ListView| {
                    view.add_child("Product ID", TextView::new(&(index+1).to_string())
                        .fixed_width(20));
                    view.add_child("Product Name", TextView::new(product_name)
                        .with_id("edit_product_name")
                        .fixed_width(20));
                    view.add_child("Product Price", TextView::new(product_price)
                        .fixed_width(20));
                    view.add_child("Quantity", EditView::new()
                        .content(qty)
                        .with_id("edit_product_qty"))
                });
        }
    }

}

fn edit_remove_popup(siv: &mut Cursive, remove: bool, inventory: bool) {
    let mut button_label = "";
    let mut title_label = "";
    if remove == true {
        title_label = "Enter Item Details to Remove";
        button_label = "Remove";
    }
    else {
        title_label = "Enter Item Details to Edit";
        button_label = "Edit";
    }
    siv.add_layer(
        Dialog::new()
            .title(title_label)
            .button(button_label, move |s| {
                let popup_product_id = s
                    .call_on_id("edit_remove_product_id", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                if popup_product_id.to_string().parse::<usize>().is_err() {
                    s.add_layer(Dialog::info("Invalid Product ID!"));
                }
                else if !in_sales_table(s, popup_product_id.to_string().clone()) {
                    s.add_layer(Dialog::info("Product ID not found!"));
                }
                else if popup_product_id.is_empty() {
                    s.add_layer(Dialog::info("Field cannot be empty!"));
                }
                else {
                    s.pop_layer();
                    edit_item(s, popup_product_id.to_string(), remove, inventory);
                }
            })
            .button("Esc|Back", |s| {
                s.pop_layer();
                let mut last_number = String::new();
                let mut number_items = String::new();
                get_inv_header(&mut last_number, &mut number_items);
                println!("\n# of items: {}\nLast Number: {}", number_items, last_number);
            })
            .content(
                ListView::new()
                    .child("Product ID", EditView::new()
                        .with_id("edit_remove_product_id")
                        .fixed_width(20))
            )
    )
}

fn init_table(siv: &mut Cursive, menu: usize) {
    let mut number_of_items = String::new();
    let mut last_number = String::new();
    let mut id = String::new();
    let mut product_name = String::new();
    let mut product_price = String::new();
    let mut buffer = [0;4];
    let mut string_buffer: Vec<u8> = Vec::new();
    let mut price_buffer = [0;8];
    let mut number_sales = 0;
    let mut total = 0.0;
    let index: isize = 0;
    let mut file: fs::File;
    match menu {
        1 => {

        },
        2 => {
            file = OpenOptions::new().read(true).open("sales.dat").unwrap();
            file.read(&mut buffer[..]);
            number_sales = u32::from_ne_bytes(buffer);
            for i in 0..number_sales {
                file.read(&mut buffer[..]);
                string_buffer = vec![0u8; u32::from_ne_bytes(buffer) as usize];
                file.read_exact(&mut string_buffer).unwrap();
                let control_number = String::from_utf8(string_buffer).unwrap();
                file.read(&mut buffer[..]);
                let number_items = u32::from_ne_bytes(buffer);
                file.seek(SeekFrom::Current((8 * number_items) as i64));
                file.read(&mut price_buffer[..]);
                let total = f64::from_ne_bytes(price_buffer);
                let total = format!("{:.2}", total);
                siv
                    .call_on_id("view_sales_table", |view: &mut LinearLayout| {
                        view.add_child(LinearLayout::horizontal()
                            .child(TextView::new(control_number)
                                .with_id(&("control_".to_string() + &i.to_string()))
                                .fixed_width(40))
                            .child(TextView::new(number_items.to_string())
                                .with_id(&("number_items_".to_string() + &i.to_string()))
                                .fixed_width(20))
                            .child(TextView::new(total)
                                .with_id(&("total_".to_string() + &i.to_string()))
                                .fixed_width(20)))
                    });
                file.seek(SeekFrom::Current(16));
            }
        },
        3 => {
            file = match fs::File::open("inventory.bak") {
                Err(why) => panic!("couldn't open: {}", why.description()),
                Ok(file) => file,
            };
            file.seek(std::io::SeekFrom::Start(8));
            get_inv_header(&mut last_number, &mut number_of_items);
            for index in 0..number_of_items.parse::<isize>().unwrap() {
                io_inv_body(&mut file, &mut id, &mut product_name, &mut product_price, false);
                println!("\n{}\n{}\n{}", id, product_name, product_price);

                siv
                    .call_on_id("manage_product_table", |view: &mut LinearLayout| {
                        let add_name = product_name.clone();
                        let mut add_price = product_price.clone();
                        let add_id = id.clone();
                        add_price = format!("{:.2}", add_price.parse::<f64>().unwrap());
                        view.add_child(LinearLayout::horizontal()
                            .child(TextView::new(add_id)
                                .with_id(&("manage_product_id_".to_string() + &(index+1).to_string()))
                                .fixed_width(40))
                            .child(TextView::new(add_name)
                                .with_id(&("manage_product_name_".to_string() + &(index+1).to_string()))
                                .fixed_width(40))
                            .child(TextView::new(add_price)
                                .with_id(&("manage_product_price_".to_string() + &(index+1).to_string()))
                                .fixed_width(10))
                            .with_id(&("manage_product_".to_string() + &(index+1).to_string())))
                    });

            }


        },
        _ => {}
    }
}

fn manage_items(siv: &mut Cursive) {
    let mut number_items = String::new();
    let mut last_number = String::new();
    get_inv_header(&mut last_number, &mut number_items);
    siv.add_global_callback(Key::Esc, |s| {s.pop_layer(); s.clear_global_callbacks(Key::Esc);});
    siv.add_layer(
        Dialog::new()
            .button("F1|Add", add_item)
            .button("F2|Edit", |s| {
                let is_empty = s.call_on_id("manage_product_table", |view: &mut LinearLayout| {
                    view.is_empty()
                });
                if  is_empty == Some(true) {
                    s.add_layer(Dialog::info("No item to edit/remove!"));
                }
                else {
                    edit_remove_popup(s, false, true);
                }

            })
            .button("F3|Remove", |s| {
                let is_empty = s.call_on_id("manage_product_table", |view: &mut LinearLayout| {
                    view.is_empty()
                });
                if  is_empty == Some(true) {
                    s.add_layer(Dialog::info("No item to edit/remove!"));
                }
                else {
                    edit_remove_popup(s, true, true);
                }
            })
            .button("Esc|Back", |s| {s.pop_layer();})
            .content(
                LinearLayout::vertical()
                    .child(LinearLayout::horizontal()
                        .child(TextView::new("Product ID")
                            .fixed_width(40))
                        .child(TextView::new("Product Name")
                            .fixed_width(40))
                        .child(TextView::new("Price")
                            .fixed_width(10)))
                    .child(BoxView::new(SizeConstraint::Fixed(95),
                                        SizeConstraint::Free,
                                        Panel::new(LinearLayout::vertical()
                                            //TODO Table Format, no child should be present upon final version
                                            //TODO children added via struct implementation
                                            .with_id("manage_product_table")))
                        .scrollable()),
            )
    );
    if number_items.parse::<u32>().unwrap() > 0 {
        init_table(siv, 3);
    }
}
