import "./Global.css"
import React from "react"

class GlobalFooter extends React.Component {

    constructor(props) {
        super(props);
    }

    render() {
        return (
            <div id="HomeFooter">
                <div>
                    <a href="/legal/userCondition">User Condition</a>
                </div>
            </div>
        );
    }

}
export default GlobalFooter;