import * as React from 'react';
import { useState, useEffect } from 'react';
import { RouteComponentProps } from '@reach/router';

import { Post } from '../../component/Post';
import { fetchApi, PostsResponse } from '../../func/api';

type Props = RouteComponentProps & {
    id?: number
};

export function Single(props: Props) {
    const [response, setResponse] = useState(null as (PostsResponse | null));

    useEffect(() => {
        let url = `/posts?from_id=${props.id}&limit=1`;
        fetchApi<PostsResponse>(url)
            .then((result) => setResponse(result))
            .catch((err) => console.error(err));
    }, [props.id]);

    return <div className="main">
        {response && response.posts.length > 0 && <Post post={response.posts[0]} />}
    </div>;
}