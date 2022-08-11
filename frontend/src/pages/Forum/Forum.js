import "./Forum.css"
import React from "react"
import ForumAPI from "api/forum.js"
import UserAPI from "api/user.js"
import GlobalFooter from "pages/Global/Footer/Global"
import GlobalHeader from "pages/Global/Header/Global"
import GlobalChatInput from "pages/Global/Input/Chat/Global.js"

class Forum extends React.Component {

    constructor(props) {
        super(props);
        this.userApi = new UserAPI();
        this.userUuid = localStorage.uuid;
        this.userApi.is_logged().then(res => {
            this.logged = res;
            this.forceUpdate();
        });
        this.forumApi = new ForumAPI();
        let words = window.location.href.split("/");
        let forumName = words[words.length - 1].replace(/%20/g, " ");
        this.forum = undefined;
        this.isSubbed = false;
        this.forumApi.get_by_name(forumName).then((res) => {
            if (res.status === 204) {
                this.forum = undefined;
            } else {
                let forum = res.data;
                if (!forum.archived) {
                    this.forum = forum;
                }
                this.forumApi.is_sub(this.userUuid, this.forum.uuid).then((res) => {
                    this.isSubbed = true;
                    this.forceUpdate();
                }).catch(_ => {
                    this.isSubbed = false;
                    this.forceUpdate();
                })
            }
            this.forceUpdate();
        });
        this.getAllChats = this.getAllChats.bind(this);
        this.subButton = this.subButton.bind(this);
        this.changeSub = this.changeSub.bind(this);
    }

    notLoggedScreen() {
        return (
            <div>
                <h1>Not logged</h1>
            </div>
        );
    }

    parseChat(chat) {
        return (
            <a href={"/chat/" + chat.uuid} className="chatBoxLink">
                <div className="chatBox">
                    <div className="chatInfo">
                        <p className="chatAuthor">By: {chat.author.name}</p>
                        <p className="chatCreation">{chat.creation_date}</p>
                    </div>
                    <h2>
                        {chat.content}
                    </h2>
                </div>
            </a>
        );
    }

    getAllChats() {
        let chats = [];
        for (let c in this.forum.chats) {
            chats.push(this.parseChat(this.forum.chats[c]));
        }
        return (
            <div>
                <GlobalChatInput fid={this.forum.uuid} author={this.userUuid} />
                <div>
                    {chats}
                </div>
            </div>
        );
    }

    async changeSub() {
        if (this.isSubbed) {
            await this.forumApi.unsub({
                "uuid": localStorage.uuid
            }, this.forum.uuid);
            this.isSubbed = false;
        } else {
            await this.forumApi.sub({
                "uuid": localStorage.uuid
            }, this.forum.uuid);
            this.isSubbed = true;
        }
        this.forceUpdate();
    }

    subButton() {
        return (
            <div>
                <button onClick={this.changeSub}>{this.isSubbed ? "UnSub" : "Sub"}</button>
            </div>
        );
    }

    render() {
        if (this.logged) {
            if (this.forum !== undefined) {
                return (
                    <div className="bodyPage">
                        <GlobalHeader></GlobalHeader>
                        <div id="ForumMainColumn">
                            <div id="ForumHeader">
                                <p></p>
                                <h1>{this.forum ? this.forum.name : ""}</h1>
                                {this.subButton()}
                            </div>
                            <div>
                                {this.getAllChats()}
                            </div>
                        </div>
                        <GlobalFooter></GlobalFooter>
                    </div>
                );
            } else {
                return (
                    <div>
                        <h1>Forum {this.forum ? this.forum.name : ""} not existing</h1>
                        <GlobalFooter></GlobalFooter>
                    </div>
                );
            }
        } else {
            return (
                <div>
                    {this.notLoggedScreen()}
                    <GlobalFooter></GlobalFooter>
                </div>
            );
        }
    }

}
export default Forum;
