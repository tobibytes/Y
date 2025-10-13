"use client"
import Post from "@/components/layouts/post"
export default function PostViewTest() {
    const posts = [{
        name: "maya",
        userName: "@maya",
        profilePic: "https://images.unsplash.com/photo-1499714608240-22fc6ad53fb2?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=76&q=80",
        postedAt: "2m ago",
        content: "Just joined Y 🌟 The UI is so clean, feels like home already!",
        media: [""],
        metrics: {likes: 10, comments: 5, shares: 3}
    }]
    return (
        <div>
            <Post {...posts[0]}/>
        </div>
    )
}