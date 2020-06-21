import * as React from 'react';
import { useState, useEffect } from 'react';
import { RouteComponentProps } from '@reach/router';

import { Post } from './Post';
import { Model } from '../../model';
import { fetchApi } from '../../func/api';

type Props = RouteComponentProps;

export function Main(props: Props) {
    const [posts, setPosts] = useState([] as Model.Post[]);

    useEffect(() => {
        fetchApi<Model.Post[]>('/posts/recent')
            .then((result) => setPosts(result))
            .catch((err) => console.error(err));
    }, []);

    return <div className="main">
        {posts.map((post) => <Post post={post} key={post.id}/>)}
    </div>;
}