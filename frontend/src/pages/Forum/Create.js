import "./Create.css"
import React from "react"
import GlobalFooter from "pages/Global/Footer/Global"
import ForumAPI from "api/forum.js";
import UserAPI from "api/user.js";
import GlobalHeader from "pages/Global/Header/Global";

class CreateForum extends React.Component {
    
    constructor(props) {
        super(props);
        this.forumApi = new ForumAPI();
        this.userApi = new UserAPI();
        this.success = <div></div>
        this.userApi.is_logged().then(res => {
            this.logged = res;
            this.forceUpdate();
        });
        this.createForumForm = {
            name: "",
            description: "",
            author_id: ""
        };
        this.changeCreateForumForm = this.changeCreateForumForm.bind(this);
        this.submitCreateForumForm = this.submitCreateForumForm.bind(this);
    }

    async submitCreateForumForm(event) {
        event.preventDefault();
        this.createForumForm.author = {
            uuid: localStorage.uuid
        };
        console.log(this.createForumForm);
        let res = await this.forumApi.create(this.createForumForm);
        console.log("res", res);
        this.success = <div id="CreateSuccessMessage">
            <p>Forum Created !</p>
            <a href={"/forum/" + this.createForumForm.name}>
                <button>See the forum</button>
            </a>
        </div>
        this.forceUpdate();
    }

    changeCreateForumForm(event) {
        switch (event.target.className) {
            case "forumName":
                this.createForumForm.name = event.target.value;
                break;
            case "forumDescription":
                this.createForumForm.description = event.target.value;
                break;
            default:
                break;
        }
    }

    render() {
        if (this.logged) {
            return (
                <div>
                    <GlobalHeader></GlobalHeader>
                    {this.success}
                    <div>
                        <h1>Create your own Forum</h1>
                    </div>
                    <div id="CreateForumDiv">
                        <form onSubmit={this.submitCreateForumForm}>
                            <div className="createForumInput">
                                <label htmlFor="forumName">Forum Name</label>
                                <input onChange={this.changeCreateForumForm} type="text" className="forumName" placeholder="forum Name"></input>
                            </div>
                            <div className="createForumInput">
                                <label htmlFor="forumDescription">Forum Description</label>
                                <textarea onChange={this.changeCreateForumForm} className="forumDescription" placeholder="forum description"></textarea>
                            </div>
                            <div className="createForumButton">
                                <input type="submit" value="Create Forum"></input>
                            </div>
                        </form>
                    </div>
                    <GlobalFooter></GlobalFooter>
                </div>
            );
        } else {
            return (
                <div>
                    <h1>Not logged</h1>
                </div>
            );
        }
    }

}
export default CreateForum;