import "./Notify.css"
import React from "react"
import UserAPI from "api/user.js"
import UnknownUser from "assets/img/UnknownUser.png"
import GlobalFooter from "pages/Global/Footer/Global"
import GlobalHeader from "pages/Global/Header/Global"

class UserNotify extends React.Component {

    constructor(props) {
        super(props);
        this.userApi = new UserAPI();
        let words = window.location.href.split("/");
        let userName = words[words.length - 2].replace(/%20/g, " ");
        let uuid = localStorage.uuid;
        this.logged = false;
        this.owner = false;
        this.userApi.get_by_name(userName).then((res) => {
            if (res.status === 204) {
                this.user = undefined;
            } else {
                this.user = res.data;
            }
            if (this.user.uuid === uuid) {
                this.owner = true;
            }
            this.forceUpdate();
        });
        this.userApi.is_logged().then(res => {
            this.logged = res;
            this.forceUpdate();
        });
        this.askingFriends = [];
        this.userApi.get_friend_asked(uuid).then((res) => {
            this.askingFriends = res.data;
            this.forceUpdate();
        });
        this.acceptFriend = this.acceptFriend.bind(this);
        this.declineFriend = this.declineFriend.bind(this);
    }

    async acceptFriend(event) {
        await this.userApi.accept_friend(localStorage.uuid, event.target.name);
        this.askingFriends = this.askingFriends.filter(function(value, index, arr) {
            if (value.uuid !== event.target.name) {
                return value;
            }
        });
        this.forceUpdate();
    }

    async declineFriend(event) {
        await this.userApi.decline_friend(localStorage.uuid, event.target.name);
        this.askingFriends = this.askingFriends.filter(function(value, index, arr) {
            if (value.uuid !== event.target.name) {
                return value;
            }
        });
        this.forceUpdate();
    }

    parseFriendsAsk() {
        let friendsAsk = [];
        for (let friend of this.askingFriends) {
            friendsAsk.push(
                <div className="askUserBloc">
                    <div className="askUserInfo">
                        <img src={UnknownUser} alt="UnknownUser" />
                        <p>{friend.name} wants to be your friend</p>
                    </div>
                    <div className="askUserFooter">
                        <div>
                            <button name={friend.uuid} onClick={this.acceptFriend}>Accept</button>
                            <button name={friend.uuid} onClick={this.declineFriend}>Decline</button>
                        </div>
                        <div>
                            {friend.creation_date}
                        </div>
                    </div>
                </div>
            );
        }
        return (friendsAsk);
    }

    checkOwner() {
        if (this.owner) {
            return (
                <div>
                    {this.parseFriendsAsk()}
                </div>
            );
        } else {
            return (
                <div>
                    <p>You are not the owner of this profile</p>
                </div>
            );
        }
    }

    checkLogged() {
        if (this.logged) {
            return (
                <div id="Notifications">
                    <div id="NotificationsOwner">
                        <h2>All your notifications</h2>
                        {this.checkOwner()}
                    </div>
                </div>
            );
        } else {
            return (
                <div>
                    <p>You must be logged in to view this page</p>
                </div>
            );
        }
    }

    render() {
        return (
            <div>
                <GlobalHeader />
                {this.checkLogged()}
                <GlobalFooter />
            </div>
        );
    }

}
export default UserNotify;