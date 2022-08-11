import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import reportWebVitals from './reportWebVitals';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Home from "./pages/Home.js";
import SignUp from "./pages/SignUp.js";
import UserCondition from "./pages/Legal/UserCondition.js";
import ForumHome from "./pages/Forum/Home.js"
import Forum from './pages/Forum/Forum.js';
import CreateForum from "./pages/Forum/Create.js";
import UserProfile from 'pages/User/Profile.js';
import UserProfileEdit from 'pages/User/Edit.js';
import UserNotify from 'pages/User/Notify.js';
import Chat from 'pages/Chat/Chat.js';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <BrowserRouter>
        <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/user/SignUp" element={<SignUp />}></Route>
            <Route path="/legal/userCondition" element={<UserCondition />}></Route>
            <Route path="/forum/home" element={<ForumHome />}></Route>
            <Route path="/forum/create" element={<CreateForum />}></Route>
            <Route path="/forum/:name" element={<Forum />}></Route>
            <Route path="/user/:name" element={<UserProfile />}></Route>
            <Route path="/user/:name/edit" element={<UserProfileEdit />}></Route>
            <Route path="/user/:name/notify" element={<UserNotify />}></Route>
            <Route path="/chat/:uuid" element={<Chat />}></Route>
        </Routes>
    </BrowserRouter>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
