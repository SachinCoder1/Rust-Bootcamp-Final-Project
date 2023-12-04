#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    price: f32,
    quantity: i32, // Can be negative if the product is sold more than in stock
}
#[derive(Debug)]
enum TransactionType {
    Sale,
    Purchase,
}

#[derive(Debug)]
struct Transaction {
    product_id: u32,
    quantity: i32,
    transaction_type: TransactionType,
}

fn add_product(products: &mut Vec<Product>, id: u32, name: String, price: f32, quantity: i32) {
    let product = Product { id, name, price, quantity };
    products.push(product);
}

fn update_product_quantity(products: &mut Vec<Product>, id: u32, quantity: i32) {
    if let Some(product) = products.iter_mut().find(|p| p.id == id) {
        product.quantity += quantity;
    }
}

fn record_transaction(transactions: &mut Vec<Transaction>, product_id: u32, quantity: i32, transaction_type: TransactionType) {
    let transaction = Transaction { product_id, quantity, transaction_type };
    transactions.push(transaction);
}




fn main() {
    let mut products: Vec<Product> = Vec::new();
    let mut transactions: Vec<Transaction> = Vec::new();

    // Sample: Adding products
    add_product(&mut products, 1, "Apple".to_string(), 0.50, 100);
    add_product(&mut products, 2, "Banana".to_string(), 0.30, 150);

    // Sample: Recording transactions
    record_transaction(&mut transactions, 1, 5, TransactionType::Sale); // Sold 5 Apples
    record_transaction(&mut transactions, 2, 10, TransactionType::Purchase); // Purchased 10 Bananas

    // Update product quantities based on transactions
    for transaction in &transactions {
        let quantity_change = match transaction.transaction_type {
            TransactionType::Sale => -transaction.quantity,
            TransactionType::Purchase => transaction.quantity,
        };
        update_product_quantity(&mut products, transaction.product_id, quantity_change);
    }

    // Displaying current inventory and transaction history
    println!("Current Inventory:");
    for product in &products {
        println!("{:?}", product);
    }

    println!("\nTransaction History:");
    for transaction in &transactions {
        println!("{:?}", transaction);
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_product() {
        let mut products: Vec<Product> = Vec::new();
        add_product(&mut products, 1, "Test Product".to_string(), 10.0, 50);

        assert_eq!(products.len(), 1);
        assert_eq!(products[0].id, 1);
        assert_eq!(products[0].name, "Test Product");
        assert_eq!(products[0].price, 10.0);
        assert_eq!(products[0].quantity, 50);
    }

    #[test]
    fn test_update_product_quantity() {
        let mut products: Vec<Product> = vec![Product { id: 1, name: "Test".to_string(), price: 10.0, quantity: 50 }];
        update_product_quantity(&mut products, 1, -10);

        assert_eq!(products[0].quantity, 40);
    }

    #[test]
    fn test_record_transaction() {
        let mut transactions: Vec<Transaction> = Vec::new();
        record_transaction(&mut transactions, 1, 5, TransactionType::Sale);

        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].product_id, 1);
        assert_eq!(transactions[0].quantity, 5);
    }
}
