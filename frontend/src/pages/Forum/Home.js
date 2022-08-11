import "./Home.css"
import React from "react"
import UserAPI from "api/user.js"
import ForumAPI from "api/forum.js"
import GlobalFooter from "pages/Global/Footer/Global.js"
import unknownUser from "assets/img/UnknownUser.png"
import GlobalSearchBar from "pages/Global/SearchBar/Global.js"
import GlobalHeader from "pages/Global/Header/Global.js";

class ForumHome extends React.Component {

    constructor(props) {
        super(props);
        this.userApi = new UserAPI();
        this.forumApi = new ForumAPI();
        this.noForumSubbed = false;
        let jwt = localStorage.jwt;
        let userName = localStorage.userName;
        this.forums = [];
        this.userApi.is_logged().then(res => {
            this.logged = res;
            this.forceUpdate();
        });
        this.forumApi.get_forum_subbed_by_user(userName).then((res) => {
            if (res.data.length === 0) {
                this.noForumSubbed = true;
            } else {
                this.noForumSubbed = false;
                this.forums = res.data;
            }
            this.forceUpdate();
        });

        this.isLogged = this.isLogged.bind(this);
        this.getForumList = this.getForumList.bind(this);
    }

    createForum(forum) {
        return (
            <div className="forum">
                <a href={"/forum/" + forum.name} className="forumLink">
                    <div className="forumInfoHeader">
                        <h2>{forum.name}</h2>
                        <p>By: {forum.author.name}</p>
                    </div>
                </a>
            </div>
        );
    }

    getForumList() {
        if (this.noForumSubbed) {
            return (
                <div>
                    <h2>Oh no... you don't have any sub to forums for now...</h2>
                </div>
            );
        } else {
            let forums = [];
            for (let forum of this.forums) {
                forums.push(this.createForum(forum))
            }
            return (
                <div id="ForumList">
                    {forums}
                </div>
            );
        }
    }

    loggedScreen() {
        return (
            <div>
                <div id="CreateNewForumBtn">
                    <img src={unknownUser}></img>
                    <a href="/forum/create">
                        <button >Create a new Forum</button>
                    </a>
                </div>
                <div id="GlobalSearch">
                    <GlobalSearchBar></GlobalSearchBar>
                </div>
                <div>
                    {this.getForumList()}
                </div>
            </div>
        );
    }

    notLoggedScreen() {
        return (
            <div>
                <h1>Not logged</h1>
            </div>
        );
    }

    isLogged() {
        if (this.logged) {
            return (this.loggedScreen());
        } else {
            return (this.notLoggedScreen());
        }
    }

    render() {
        return (
            <div>
                <GlobalHeader />
                <div>
                    <h1>Forum Home</h1>
                </div>
                <div>
                    {this.isLogged()}
                </div>
                <GlobalFooter></GlobalFooter>
            </div>
        )
    }

}
export default ForumHome;