enum OrderStatus {
    Pending, 
    Shipped,
    Delivered,
    Cancelled,
}

enum PaymentMethod {
    CreditCard,
    DebitCard,
    Paypal,
    CashOnDelivery,
}

struct Order {
    id: u32,
    status: OrderStatus,
    payment: PaymentMethod,
}

fn process_order_status(order: &mut Order) {
    match order.status {
        OrderStatus::Pending => {
            println!("Order is now being shipped");
            order.status = OrderStatus::Shipped;
        },
        OrderStatus::Shipped => {
            println!("Order has been delivered");
            order.status = OrderStatus::Delivered;
        },
        OrderStatus::Delivered | OrderStatus::Cancelled => {
            println!("No further action needed");
        },
    }
}

fn print_payment_method(payment: &PaymentMethod) {
    match payment {
        PaymentMethod::CreditCard => {
            println!("Payment by credit card");
        },
        PaymentMethod::DebitCard => {
            println!("Payment by debit card");
        }, 
        PaymentMethod::Paypal => {
            println!("Payment by Paypal");
        },
        PaymentMethod::CashOnDelivery => {
            ("Payment by cash on delivery");
        },
    }       
}

fn apply_discount(discount_code: Option<&str>) {
    if let Some(code) = discount_code {
        println!("Discount code applied: {}", code);
    } else {
        println!("No discount applied");
    }
}

fn main() {
    let mut order = Order {
        id: 1,
        status: OrderStatus::Pending,
        payment: PaymentMethod::CreditCard,
    };
    
    process_order_status(&mut order);
    process_order_status(&mut order);
    process_order_status(&mut order);
    
    print_payment_method(&order.payment);
    
    let discount_code = Some("Butt");
    apply_discount(discount_code);
    
    let no_discount_code: Option<&str> = None;
    apply_discount(no_discount_code);
}












