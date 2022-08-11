import "./Chat.css"
import React from "react"
import GlobalHeader from "pages/Global/Header/Global";
import GlobalFooter from "pages/Global/Footer/Global";
import ForumAPI from "api/forum.js";
import UserAPI from "api/user.js";
import GlobalChatInput from "pages/Global/Input/Chat/Global.js";

class Chat extends React.Component {

    constructor(props) {
        super(props);
        this.userApi = new UserAPI();
        this.forumApi = new ForumAPI();
        let words = window.location.href.split("/");
        this.chatUuid = words[words.length - 1];
        this.userUuid = localStorage.uuid;
        this.fid = undefined;
        this.chat = {
            author: {},
            answers: []
        };
        this.userApi.is_logged().then(res => {
            this.logged = res;
            this.forceUpdate();
        });
        this.forumApi.get_chat_by_uuid(this.chatUuid).then(res => {
            this.chat = res.data;
            this.fid = this.chat.forum.uuid;
            this.forceUpdate();
            
        });
        this.getChats = this.getChats.bind(this);
    }

    parseChat(chat) {
        return (
            <a href={"/chat/" + chat.uuid} className="CchatBoxLink">
                <div className="CchatBox">
                    <div className="CchatInfo">
                        <p className="CchatAuthor">By: {chat.author.name}</p>
                        <p className="CchatCreation">{chat.creation_date}</p>
                    </div>
                    <h2>
                        {chat.content}
                    </h2>
                </div>
            </a>
        );
    }

    getChats() {
        let chats = [];
        for (let chat of this.chat.answers) {
            chats.push(
                this.parseChat(chat)
            );
        }
        return chats;
    }

    render() {
        if (this.fid === undefined) {
            return (
                <div>
                    <GlobalHeader />
                    <h1>Loading...</h1>
                    <GlobalFooter />
                </div>
            );
        } else {
            return (
                <div>
                    <GlobalHeader></GlobalHeader>
                    <div id="CChatBox">
                        <div id="CMainChat">
                            {this.parseChat(this.chat)}
                        </div>
                        <GlobalChatInput fid={this.fid} author={this.userUuid} answer={this.chatUuid} />
                        <h2>Réponses:</h2>
                        <div id="CChatAnswers">
                            {this.getChats()}
                        </div>
                    </div>
                    <GlobalFooter></GlobalFooter>
                </div>
            );
        }
    }

}
export default Chat;
