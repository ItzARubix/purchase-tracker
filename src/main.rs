use bincode::{config, Decode, Encode};
use std::fs::File;
use std::env::args;
use std::io;

#[derive(Encode, Decode, Debug)]
struct Date {
    month: u8,
    day: u8,
    year: u64
}

impl ToString for Date {
    fn to_string(&self) -> String {
        let mut temp: String = "".to_string();
        temp.push_str(&self.month.to_string());
        temp.push('/');
        temp.push_str(&self.day.to_string());
        temp.push('/');
        temp.push_str(&self.year.to_string());
        temp
    }
}

fn date_cli() -> Date {
    let stdin = io::stdin();
    let mut date_as_string = "".to_string();
    match stdin.read_line(&mut date_as_string) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }
    let mut month: u8 = 0;
    let mut day: u8 = 0;
    let mut year: u64 = 0;
    for (index, i) in date_as_string.trim().split('/').enumerate() {
        match index {
            0 => {
                month = match i.trim().parse::<u8>() {
                    Ok(x) => x,
                    Err(x) => panic!("Failed to parse your input as a date. Maybe you didn't correctly write your input as \"MM/DD/YYYY\"?\nHere's the reported error, if it helps: {}", x)
                }
            },
            1 => {
                day = match i.trim().parse::<u8>() {
                    Ok(x) => x,
                    Err(x) => panic!("Failed to parse your input as a date. Maybe you didn't correctly write your input as \"MM/DD/YYYY\"?\nHere's the reported error, if it helps: {}", x)
                }
            },
            2 => {
                year = match i.trim().parse::<u64>() {
                    Ok(x) => x,
                    Err(x) => panic!("Failed to parse your input as a date. Maybe you didn't correctly write your input as \"MM/DD/YYYY\"?\nHere's the reported error, if it helps: {}", x)
                }
            }
            _ => println!("Ignoring errenous additional date field: {}", i)
        }
    }
    if month == 0 || day == 0 || year == 0 {
        panic!("Failed to parse your input as a date. Maybe you didn't correctly write your input as \"MM/DD/YYYY\"? Note that zeros in any of the fields are unsupported.")
    }
    Date {
        month,
        day,
        year
    }
}

#[derive(Encode, Decode, Debug)]
struct Product {
    name: String, // Name of the product
    desc: String, // Description of the product
    base_price: u64, // Price of product excluding additional costs incurred by add-ons
    sticker_price: u64, // Price of product in cents
    items: Option<Vec<Product>>, // Itemized cost of product (if applicable)
    // NOTE:
    // Generally, the base_price cannot be easily determined by summing up the costs of the
    // Products in the items Vec
    // Although it is sometimes the case that the total cost of the product is the sum of the
    // individual items, often vendors will try to make bundles a "value offer" by making their
    // price be less than the price of buying all the items individually, independent from the
    // discounts that get accounted for in the paid_amount.
    add_ons: Option<Vec<AddOn>> // List of add-ons (if applicable)
}

fn product_cli() -> Product {
    let stdin = io::stdin();

    println!("Product name?");
    let mut name = "".to_string();
    match stdin.read_line(&mut name) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }

    println!("Product description?");
    let mut desc = "".to_string();
    match stdin.read_line(&mut desc) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }

    println!("What's the price of the product in cents excluding add-ons and sales?");
    let mut temp = "".to_string();
    match stdin.read_line(&mut temp) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }
    let base_price = match temp.trim().parse::<u64>() {
        Ok(x) => x,
        Err(x) => panic!("Failed to parse your input as a number. Your input should be an unsigned integer, no negative sign or decimal, and certainly no nonnumeric characters. Here's the reported error, if it helps: {}", x)
    };

    println!("What's the price of the product in cents including add-ons but excluding sales?");
    let mut temp = "".to_string();
    match stdin.read_line(&mut temp) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }
    let sticker_price = match temp.trim().parse::<u64>() {
        Ok(x) => x,
        Err(x) => panic!("Failed to parse your input as a number. Your input should be an unsigned integer, no negative sign or decimal, and certainly no nonnumeric characters. Here's the reported error, if it helps: {}", x)
    };

    println!("Does {} have one or more subitems? (Type \"Yes\" or \"No\")", name);
    let items: Option<Vec<Product>>;
    loop {
        let mut temp = "".to_string();
        match stdin.read_line(&mut temp) {
            Ok(_) => (),
            Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
        }
        if temp.trim() == "Yes" {
            println!("How many subitems?");
            temp = "".to_string();
            match stdin.read_line(&mut temp) {
                Ok(_) => (),
                Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
            }
            let mut tempsubitems:Vec<Product> = vec!();
            let repetitions = match temp.trim().parse::<u64>() {
                Ok(x) => x,
                Err(x) => panic!("Failed to parse your input as a number. Your input should be an unsigned integer, no negative sign or decimal, and certainly no nonnumeric characters. Here's the reported error, if it helps: {}", x)
            };
            for i in 0..repetitions {
                println!("{}th subitem of {}:\n", i, name);
                tempsubitems.push(product_cli());
            }
            items = Some(tempsubitems);
            break;
        }
        if temp.trim() == "No" {
            items = None;
            break;
        }
        println!("Please type \"Yes\" or \"No\"");
    }

    println!("Does {} have one or more add-ons? (Type \"Yes\" or \"No\")", name);
    let add_ons: Option<Vec<AddOn>>;
    loop {
        let mut temp = "".to_string();
        match stdin.read_line(&mut temp) {
            Ok(_) => (),
            Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
        }
        if temp.trim() == "Yes" {
            println!("How many add-ons?");
            temp = "".to_string();
            match stdin.read_line(&mut temp) {
                Ok(_) => (),
                Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
            }
            let mut tempsubitems:Vec<AddOn> = vec!();
            let repetitions = match temp.trim().parse::<u64>() {
                Ok(x) => x,
                Err(x) => panic!("Failed to parse your input as a number. Your input should be an unsigned integer, no negative sign or decimal, and certainly no nonnumeric characters. Here's the reported error, if it helps: {}", x)
            };
            for i in 0..repetitions {
                println!("{}th add-on of {}:\n", i, name);
                tempsubitems.push(add_on_cli());
            }
            add_ons = Some(tempsubitems);
            break;
        }
        if temp.trim() == "No" {
            add_ons = None;
            break;
        }
        println!("Please type \"Yes\" or \"No\"");
    }
    Product {
        name,
        desc,
        base_price,
        sticker_price,
        items,
        add_ons
    }
}

impl ToString for Product {
    fn to_string(&self) -> String {
        let mut temp = "".to_string();
        temp.push_str("Name: ");
        temp.push_str(&self.name);
        temp.push_str("\nDescription: ");
        temp.push_str(&self.desc);
        temp.push_str("\nBase Price: $");
        temp.push_str(&(self.base_price/100).to_string());
        temp.push('.');
        temp.push_str(&(self.base_price%100).to_string());
        temp.push_str("\nSticker Price: $");
        temp.push_str(&(self.sticker_price/100).to_string());
        temp.push('.');
        temp.push_str(&(self.sticker_price%100).to_string());
        temp.push('\n');
        match &self.items {
            None => (),
            Some(x) => {
                temp.push_str("Sub-products: {\n\n");
                for (index, item) in x.iter().enumerate() {
                    temp.push_str(&index.to_string());
                    temp.push_str(".\n");
                    temp.push_str(&item.to_string());
                    temp.push_str("\n\n");
                }
                temp.push_str("}\n");
            }
        }
        match &self.add_ons {
            None => (),
            Some(x) => {
                temp.push_str("Add-ons: {\n\n");
                for (index, item) in x.iter().enumerate() {
                    temp.push_str(&index.to_string());
                    temp.push_str(".\n");
                    temp.push_str(&item.to_string());
                    temp.push_str("\n\n");
                }
                temp.push_str("}\n");
            }
        }
        temp
    }
}

#[derive(Encode, Decode, Debug)]
struct AddOn {
    name: String, // Name of the add-on
    desc: String, // Description of the add-on
    sticker_price: u64, // Price of the add-on
    actual_price: u64, // Price of the add-on after discounts
    // NOTE:
    // If an add-on has an associated product but the add-on costs less than the associated
    // product, do NOT but the price of the associated product in sticker_price and then price of
    // the add-on in paid_amount.
    // The variable sticker_price is for what the "usual" cost of the ADD-ON is, not the usual
    // price of the product it's coming from. THEN, if the add-on itself is on sale, put the price
    // of the add-on after discounts in paid_ammount
    assoc_product: Option<Product> // Product associated with add-on (if applicable)
}

fn add_on_cli() -> AddOn {
    let stdin = io::stdin();

    println!("Add-on name?");
    let mut name = "".to_string();
    match stdin.read_line(&mut name) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }

    println!("Add-on description?");
    let mut desc = "".to_string();
    match stdin.read_line(&mut desc) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }

    println!("Typical price of the add-on in cents? (This is the nondiscounted price of the add-on, not the typical price of the associated product if one exists)");
    let mut temp = "".to_string();
    match stdin.read_line(&mut temp) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }
    let sticker_price = match temp.trim().parse::<u64>() {
        Ok(x) => x,
        Err(x) => panic!("Failed to parse your input as a number. Your input should be an unsigned integer, no negative sign or decimal, and certainly no nonnumeric characters. Here's the reported error, if it helps: {}", x)
    };

    println!("What's the price of the add-on after discounts in cents? (Usually, this is the amount that the product that you added this onto increased by)");
    let mut temp = "".to_string();
    match stdin.read_line(&mut temp) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }
    let actual_price = match temp.trim().parse::<u64>() {
        Ok(x) => x,
        Err(x) => panic!("Failed to parse your input as a number. Your input should be an unsigned integer, no negative sign or decimal, and certainly no nonnumeric characters. Here's the reported error, if it helps: {}", x)
    };

    let assoc_product:Option<Product>;
    println!("Does {} have an associated product? (Type \"Yes\" or \"No\")", name);
    loop {
        let mut temp = "".to_string();
        match stdin.read_line(&mut temp) {
            Ok(_) => (),
            Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
        }
        if temp.trim() == "Yes" {
            println!("Please enter information about the associated product:\n");
            assoc_product = Some(product_cli());
            break;
        }
        if temp.trim() == "No" {
            assoc_product = None;
            break;
        }
        println!("Please type \"Yes\" or \"No\"");
    }

    AddOn {
        name,
        desc,
        sticker_price,
        actual_price,
        assoc_product
    }

}

impl ToString for AddOn {
    fn to_string(&self) -> String {
        let mut temp = "Name: ".to_string();
        temp.push_str(&self.name);
        temp.push_str("\nDescription: ");
        temp.push_str(&self.desc);
        temp.push_str("\nSticker Price: $");
        temp.push_str(&(self.sticker_price/100).to_string());
        temp.push('.');
        temp.push_str(&(self.sticker_price%100).to_string());
        temp.push('\n');
        temp.push_str("\nActual Price: $");
        temp.push_str(&(self.actual_price/100).to_string());
        temp.push('.');
        temp.push_str(&(self.actual_price%100).to_string());
        match &self.assoc_product {
            None => (),
            Some(x) => {
                temp.push_str("\nAssociated Product: {\n\n");
                temp.push_str(&x.to_string());
                temp.push_str("\n}");
            }
        }
        temp
    }
}

#[derive(Encode, Decode, Debug)]
struct Order {
    // I was originally going to include a sticker_price field, which was the sum of the
    // paid_amount fields for all of the products that made up the order
    // But then I realized that this was dumb because the Vec<Product> is stored in this struct
    // anyway so we'd be adding a really random invariant that doesn't need to exist because we can
    // just calculate the sticker_price from the Order itself.
    // So instead I'm just going to include the subtotal (which is the price after discounts before
    // shipping and taxes) and the total, which is the price actually paid.
    date_placed: Date,
    date_shipped: Date,
    subtotal: u64,
    total: u64,
    products: Vec<(Product, u64)>, // The u64 here represents the actual amount paid for that
                                   // particular item in the order, different from the stored
                                   // sticker price in that this tuple element should account for
                                   // item-specific discounts, while the sticker price is merely
                                   // the cost after add-ons
    notes: String
}
/*
 * To put it simply, Product.base_price is the price of the product excluding add-ons,
 * Product.sticker_price is the price of the product including add-ons but excluding item-specific
 * discounts, such as a sale for that particular item, and the u64 stored in the tuple in the Vec
 * in the Order is the actual price for that particular item after sale discounts that affect
 * individual items. Then, Order.subtotal is the cost of your order after coupons and other "full
 * order" discounts (if you have no coupons, then it is more than likely that Order.subtotal will
 * just be the sum of the u64s in the Vec of tuples). Lastly, Order.total is the amount that you
 * actually paid for that order, after shipping and taxes and whatnot. The amount of money that was
 * removed from your bank account for that transaction is Order.total :3 makes sense?
 */

impl ToString for Order {
    fn to_string(&self) -> String {
        let mut temp = "Placed: ".to_string();
        temp.push_str(&self.date_placed.to_string());
        temp.push_str("\nShipped: ");
        temp.push_str(&self.date_shipped.to_string());
        temp.push_str("\nSubtotal: $");
        temp.push_str(&(self.subtotal/100).to_string());
        temp.push('.');
        temp.push_str(&(self.subtotal%100).to_string());
        temp.push_str("\nTotal: $");
        temp.push_str(&(self.total/100).to_string());
        temp.push('.');
        temp.push_str(&(self.total%100).to_string());
        temp.push_str("\nProducts: {\n\n");
        for (index, (product, price)) in self.products.iter().enumerate() {
            temp.push_str(&index.to_string());
            temp.push_str(". $");
            temp.push_str(&(price/100).to_string());
            temp.push('.');
            temp.push_str(&(price%100).to_string());
            temp.push('\n');
            temp.push_str(&product.to_string());
            temp.push('\n');
        }
        temp
    }
}

fn order_cli() -> Order {
    let stdin = io::stdin();
    println!("Welcome to the order creator!");
    println!("Please type the date you placed this order.");
    println!("Use \"MM/DD/YYYY\" format. Note that malformed input may crash the program, but illegal dates will not be rejected.");
    let date_placed = date_cli();
    println!("Please type the date that this order was shipped.");
    println!("Use \"MM/DD/YYYY\" format. Note that malformed input may crash the program, but illegal dates will not be rejected.");
    let date_shipped = date_cli();
    println!("Enter the subtotal (this should be the price of the order after coupons and sale discounts but before shipping and taxes) in cents.");
    let mut temp = "".to_string();
    match stdin.read_line(&mut temp) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }
    let subtotal = match temp.trim().parse::<u64>() {
        Ok(x) => x,
        Err(x) => panic!("Failed to parse your input as a number. Your input should be an unsigned integer, no negative sign or decimal, and certainly no nonnumeric characters. Here's the reported error, if it helps: {}", x)
    };

    println!("Enter the total (this should be the amount of money you actually paid) in cents.");
    let mut temp = "".to_string();
    match stdin.read_line(&mut temp) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }
    let total = match temp.trim().parse::<u64>() {
        Ok(x) => x,
        Err(x) => panic!("Failed to parse your input as a number. Your input should be an unsigned integer, no negative sign or decimal, and certainly no nonnumeric characters. Here's the reported error, if it helps: {}", x)
    };
    println!("How many products did you order?");
    let mut temp = "".to_string();
    match stdin.read_line(&mut temp) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }
    let number_of_products = match temp.trim().parse::<u64>() {
        Ok(x) => x,
        Err(x) => panic!("Failed to parse your input as a number. Your input should be an unsigned integer, no negative sign or decimal, and certainly no nonnumeric characters. Here's the reported error, if it helps: {}", x)
    };

    let mut products: Vec<(Product, u64)> = vec!();
    for i in 0..number_of_products {
        println!("Product {}", i);
        let temp_product = product_cli();
        println!("What was the cost (in cents) of this product accounting for all add-ons and sales?");
        let mut temp = "".to_string();
        match stdin.read_line(&mut temp) {
            Ok(_) => (),
            Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
        }
        let temp_price = match temp.trim().parse::<u64>() {
            Ok(x) => x,
            Err(x) => panic!("Failed to parse your input as a number. Your input should be an unsigned integer, no negative sign or decimal, and certainly no nonnumeric characters. Here's the reported error, if it helps: {}", x)
        };
        products.push((temp_product, temp_price));
    }

    println!("Any other notes for your order?");
    let mut notes = "".to_string();
    match stdin.read_line(&mut notes) {
        Ok(_) => (),
        Err(x) => panic!("Reading from the Standard Input failed. I have no idea why that happened, but here's the error if it's helpful to you: {}", x)
    }

    Order {
        date_placed,
        date_shipped,
        subtotal,
        total,
        products,
        notes
    }
}


fn main() {
    // Get the commandline arguments
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        panic!("Usage: purchase_tracker [OUTFILE] | purchase_tracker [INFILE] [OUTFILE]\nRun 'purchase_tracker --help' for more information");
    }

    // Help message
    if args[1] == "--help" {
        println!("Usage: purchase_tracker [OUTFILE] | purchase_tracker [INFILE] [OUTFILE]");
        println!("In the first case (NEW mode), where only one file argument is passed, a new list of orders (initially empty, but populated with contents provided during the usage of that particular session of purchase_tracker) is saved to OUTFILE. If OUTFILE already exists, purchase_tracker will err.");
        println!("In the second case (UPDATE mode), a list of orders is read from INFILE and (after any new orders are added) saved to OUTFILE. INFILE must already exist, and OUTFILE must not; purchase_tracker will err if INFILE doesn't exist or OUTFILE does already.");
        println!("Saving the updated orders list to the same file as it was read from (i.e. setting INFILE and OUTFILE to be the same file) is unsupported. Doing so may result in the file being truncated, deleting all orders.");
        println!("Example: purchase_tracker file.txt new_file.txt");
        println!("Will read 'file.txt' and print all orders that were saved to that file (will fail if 'file.txt' does not exist). You will then be brought to a CLI where you can construct a new order and add it to the list. The resulting updated order list is saved to 'new_file.txt' before quitting the program.");
        println!("NOTE: For all intents and purposes, any usage of UPDATE mode will fail if the input file was not created with this program.\nOnly try to read a file with '-r' if that file was created using '-w' during a past run of purchase_tracker. When using UPDATE mode, ensure that INFILE was created with a previous run of this program.");
        println!("If you have no files that were created with a previous run of the program, run the program in NEW mode to create one.");
        return;
    }

    // Configuration for bincode
    let config = config::standard();

    match args.len() {
        1 => {
            unreachable!("Usage: purchase_tracker [OUTFILE] | purchase_tracker [INFILE] [OUTFILE]\nRun 'purchase_tracker --help' for more information");
        },
        2 => { // NEW mode
            let mut outfile = match File::create_new(&args[1]) { // Panic if the file already exists.
                Ok(x) => x,
                Err(x) => panic!("The file {} probably already exists (or you don't have permission to create it) (or the parent directory of the file you entered doesn't exist yet). Run purchase_tracker --help for more information.\nHere's the error that was received upon trying to create the file: {}", args[1], x),
            };
            let the_output: Vec<Order> = vec!(order_cli());
            match bincode::encode_into_std_write(the_output, &mut outfile, config) {
                Ok(_) => {
                    println!("Success! The encoded order is saved to {}. In the future, run purchase_tracker in UPDATE mode using that file as the INFILE in order to add new orders.", args[1])
                },
                Err(x) => {
                    panic!("Well, this is awkward...\nDespite the fact that {} was able to be opened successfully, the program failed to save your order to the file. Here's the error that was reported: {}", args[1], x);
                }
            };
        },
        3 => { // UPDATE mode
            let mut outfile = match File::create_new(&args[2]) { // Panic if the file already exists.
                Ok(x) => x,
                Err(x) => panic!("The file {} probably already exists (or you don't have permission to create it) (or the parent directory of the file you entered doesn't exist yet). Run purchase_tracker --help for more information.\nHere's the error that was received upon trying to create the file: {}", args[2], x),
            };
            let mut infile = match File::open(&args[1]) {
                Ok(x) => x,
                Err(x) => panic!("The file {} probably doesn't exist (or you don't have permission to read it). Run purchase_tracker --help for more information.\nHere's the error that was received upon trying to create the file: {}", args[1], x),
            };
            let mut the_output: Vec<Order> = match bincode::decode_from_std_read(&mut infile, config) {
                Ok(x) => x,
                Err(x) => panic!("Failed to decode from {}.\nThis probably means that the file wasn't created with a previous run of purchase_tracker.\nHere's the error that was reported: {}", args[1], x)
            };

            println!("Your orders:");
            for (index, order) in the_output.iter().enumerate() {
                println!("{}.\n{}", index, order.to_string())
            }

            the_output.push(order_cli());
            match bincode::encode_into_std_write(the_output, &mut outfile, config) {
                Ok(_) => {
                    println!("Success! The encoded order is saved to {}. In the future, run purchase_tracker in UPDATE mode using that file as the INFILE in order to add new orders.", args[2])
                },
                Err(x) => {
                    panic!("Well, this is awkward...\nDespite the fact that {} was able to be opened successfully, the program failed to save your order to the file. Here's the error that was reported: {}", args[2], x);
                }
            };
        },
        _ => panic!("Usage: purchase_tracker [OUTFILE] | purchase_tracker [INFILE] [OUTFILE]\nRun 'purchase_tracker --help' for more information")
    };

    println!("If you're reading this, that hopefully means that purchase_tracker ran successfully :3 please tell Nyl anything about the app that you'd like, he always wants to hear about your experience <3");

//                Ok(_) => {println!("Success! The encoded order is saved to {}. In the future, run purchase_tracker in UPDATE mode using that file as the INFILE in order to add new orders.", args[1])},
    /*
    // Check for malformed input
    if args.len() < 3 {
        panic!("Usage: purchase_tracker [OUTFILE] | purchase_tracker [INFILE] [OUTFILE]\nRun 'purchase_tracker --help' for more information");
    }

    // Check that the user provided a valid read/write mode
    let write_mode:bool = if args[1] == "-r" {
        false
    } else if args[1] == "-w" {
        true 
    } else {
        panic!("Usage: purchase_tracker [OUTFILE] | purchase_tracker [INFILE] [OUTFILE]\nRun 'purchase_tracker --help' for more information");
    };

    // If we're in read mode, the file we're trying to read must exist
    if !write_mode {
        match std::fs::exists(&args[2]) {
            Ok(true) => (),
            Ok(false) => {
                panic!("File {} does not exist", args[2]);
            },
            Err(x) => {
                panic!("Failed to verify that file {} exists\nIf it helps, the error in question was: {}", args[2], x);
            }
        }
    }
    /*
    } else if write_mode { // If we're in write mode, the file should be backed up if it exists
        match std::fs::exists(&args[2]) {
            Ok(true) => {
                let unix_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to fetch system time. You should check your system's clock\nIf you're wondering why I need the system clock, it's for giving the backup file a unique name");
                let backup_name = format!(".backup_{}_{}", args[2], unix_timestamp.as_secs());
                match File::create_new(&backup_name) {
                    Err(x) => {
                        panic!("Failed to create the backup file.\nThis error should never happen in practice, as it means that a file named {} already exists\nTry rerunning the program to see if that fixes it\nIf it helps, the error in question was: {}", backup_name ,x);
                    },
                    _ => todo!()
                }
            },
            Ok(false) => todo!(),
            Err(x) => todo!()
        }
    }
    } else {
        unreachable!("Program panicked with unknown error\nTell Nyl about this, and include 'URID1' in your error log");
    }
    */

    // Set up the standard bincode config
    let config = config::standard();
    */

}
