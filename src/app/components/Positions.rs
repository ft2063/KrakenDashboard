use leptos::{*, html::Div};

#[component]
pub fn Positions() -> impl IntoView {
    view! {
        <div>
            <h1>"Positions"</h1>
            <table>
                <thead>
                    <tr>
                        <th>"MARKET"</th>
                        <th>"SIZE"</th>
                        <th>"ENTRY / MARK"</th>
                        <th>"EST. LIQ. PRICE"</th>
                        <th>"EFFECTIVE LVG"</th>
                        <th>"IM / MM"</th>
                        <th>"PNL / ROE"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Example Market"</td>
                        <td>"Example Size"</td>
                        <td>"Example Entry / Mark"</td>
                        <td>"Example Est. Liquidity Price"</td>
                        <td>"Example Effective LVG"</td>
                        <td>"Example IM / MM"</td>
                        <td>"Example PNL / ROE"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
