import "./Global.css"
import React from "react"
import ForumAPI from "api/forum.js"

class GlobalFooter extends React.Component {

    constructor(props) {
        super(props);
        this.forumApi = new ForumAPI();
        this.searchForm = {
            name: ""
        }
        this.formResult = [];
        this.submitForm = this.submitForm.bind(this);
        this.changeForm = this.changeForm.bind(this);
        this.getForumList = this.getForumList.bind(this);
    }

    submitForm(event) {
        event.preventDefault();
    }

    async changeForm(event) {
        this.searchForm.name = event.target.value;
        let res = await this.forumApi.search_by_name(this.searchForm.name);
        this.formResult = res.data;
        this.forceUpdate();
    }

    getForumList() {
        let res = [];
        for (let forum of this.formResult) {
            res.push(
                <a href={"/forum/" + forum.name} className="forumSearchClickable">
                    <div className="forumSearch">
                        <p>{forum.name}</p>
                    </div>
                </a>
            );
        }
        return (res);
    }

    render() {
        return (
            <div>
                <div id="SearchBar">
                    <form onSubmit={this.submitForm}>
                        <div>
                            <label htmlFor="searchBarName">Forum name</label>
                            <input onChange={this.changeForm}></input>
                        </div>
                    </form>
                </div>
                <div>
                    {this.getForumList()}
                </div>
            </div>
        );
    }

}
export default GlobalFooter;