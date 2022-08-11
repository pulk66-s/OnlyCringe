import "./Global.css"
import React from "react"
import UnknownUser from "assets/img/UnknownUser.png"
import UserAPI from "api/user.js"

class GlobalHeader extends React.Component {

    constructor(props) {
        super(props);
        this.userApi = new UserAPI();
        this.userUuid = localStorage.uuid;
        this.profileSrc = UnknownUser;
        this.test = "test";
        this.userApi.get_profile_picture(this.userUuid).then(res => {
            if (res.status === 202) {
                this.profileSrc = "data:image/png;base64," + res.data;
                this.forceUpdate();
            }
        });
        this.clicked = false;
        this.switchClick = this.switchClick.bind(this);
    }

    switchClick() {
        this.clicked = !this.clicked;
        this.forceUpdate();
    }

    disconnect() {
        localStorage.clear();
        window.location.href = "/";
    }

    render() {
        return (
            <div id="GlobalHeader">
                <div></div>
                <div id="HeaderTitle">
                    <a href="/">
                        <h1>OnlyCringes</h1>
                    </a>
                </div>
                <div id="LogoMenu" className={this.clicked ? "logoMenuVisible" : "logoMenuUnVisible"}>
                    <img onClick={this.switchClick} id="HeaderUserLogo" src={this.profileSrc} alt="Unknown User" />
                    <div>
                        <a href={"/user/" + this.userUuid}>
                            <div id="HeaderMenuProfile" className="profileMenuTile menuTile">
                                <img src={this.profileSrc}></img>
                                <p>Profile</p>
                            </div>
                        </a>
                        <a onClick={this.disconnect}>
                            <div className="menuTile">
                                <p>Disconnect</p>
                            </div>
                        </a>
                    </div>
                </div>
            </div>
        );
    }

}
export default GlobalHeader;