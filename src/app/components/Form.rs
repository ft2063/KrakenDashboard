use leptos::{*, html::Div};

#[component]
pub fn Form() -> impl IntoView {
    view! {
        <div class="container">
            <h1 class="title">"Order Form"</h1>
            <form class="order-form">
                <div class="button-group">
                    <button type="button" class="btn sell-btn">"Sell"</button>
                    <button type="button" class="btn buy-btn">"Buy"</button>
                </div>
                <div class="quantity-group">
                    <label for="quantity">"Quantity"</label>
                    <input type="number" id="quantity" name="quantity" class="quantity-input" placeholder="Enter quantity" />
                </div>
                <button type="submit" class="place-order-btn">"Place Order"</button>
            </form>
        </div>
    }
}

