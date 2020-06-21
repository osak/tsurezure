import * as React from 'react';
import { useState, useEffect } from 'react';
import { RouteComponentProps, Link, useLocation } from '@reach/router';

import { Post } from '../../component/Post';
import { fetchApi, PostsResponse } from '../../func/api';
import { Paginator } from './Paginator';

type Props = RouteComponentProps;

export function Main(props: Props) {
    const [response, setResponse] = useState(null as (PostsResponse | null));
    const location = useLocation();
    const urlParams = new URLSearchParams(location.search);
    const fromParam = urlParams.get('from');

    useEffect(() => {
        let url = '/posts?limit=20';
        if (fromParam != undefined) {
            url += `&from=${fromParam}`
        }
        fetchApi<PostsResponse>(url)
            .then((result) => setResponse(result))
            .catch((err) => console.error(err));
    }, [fromParam]);

    return <div className="main">
        {response && response.posts.map((post) => <Post post={post} key={post.id}/>)}
        <nav className="nav">
            <Paginator response={response} />
        </nav>
    </div>;
}