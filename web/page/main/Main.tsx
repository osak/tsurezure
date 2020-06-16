import * as React from 'react';
import { useState, useEffect } from 'react';
import { Post } from './Post';
import { Model } from '../../model';
import { fetchApi } from '../../func/api';

export function Main() {
    const [posts, setPosts] = useState([] as Model.Post[]);

    useEffect(() => {
        fetchApi<Model.Post[]>('/posts/recent')
            .then((result) => setPosts(result))
            .catch((err) => console.error(err));
    }, []);

    return <div>
        {posts.map((post) => <Post post={post} key={post.id}/>)}
    </div>;
}