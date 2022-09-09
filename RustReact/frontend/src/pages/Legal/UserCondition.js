import "./UserCondition.css"
import React from "react"

class UserCondition extends React.Component {

    constructor(props) {
        super(props);
    }

    render() {
        return (
            <div>
                <div>
                    <h1>User Condition</h1>
                </div>
                <div>
                    <a href="/">
                        <button>Back To Home</button>
                    </a>
                </div>
            </div>
        );
    }

}
export default UserCondition;