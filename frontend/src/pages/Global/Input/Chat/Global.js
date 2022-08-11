import "./Global.css"
import React from "react"
import ForumAPI from "api/forum.js"

class GlobalChatInput extends React.Component {

    constructor(props) {
        super(props);
        const params = this.props;
        this.fid = params.fid;
        this.author = params.author;
        this.answer = params.answer;
        this.forumApi = new ForumAPI();
        this.createChatForm = {
            content: ""
        };
        this.createChatChange = this.createChatChange.bind(this);
        this.submitChat = this.submitChat.bind(this);
    }

    async submitChat(event) {
        event.preventDefault();
        let data = {};
        if (this.author !== undefined) {
            if (this.answer === undefined) {
                data = {
                    "content": this.createChatForm.content.trim(),
                    "forum": {
                        "uuid": this.fid
                    },
                    "author": {
                        "uuid": this.author
                    }
                };
            } else {
                data = {
                    "content": this.createChatForm.content.trim(),
                    "forum": {
                        "uuid": this.fid
                    },
                    "author": {
                        "uuid": this.author
                    },
                    "answer_to": {
                        "uuid": this.answer
                    }
                };
            }
            if (data.content.length > 0) {
                await this.forumApi.create_chat(data);
            }
        }
        window.location.reload();
    }

    createChatChange(event) {
        switch (event.target.name) {
            case "chatContent":
                this.createChatForm.content = event.target.value;
                break;
            default:
                break;
        }
    }

    render() {
        return (
            <div id="CreateChat">
                <form onSubmit={this.submitChat}>
                    <div id="createChatContentField">
                        <label htmlFor="createChat">Post your Chat</label>
                        <textarea name="chatContent" onChange={this.createChatChange} id="CreateChatContent"></textarea>
                    </div>
                    <div id="createChatSubmitField">
                        <input type="submit" value="Post"></input>
                    </div>
                </form>
            </div>
        );
    }

}
export default GlobalChatInput;