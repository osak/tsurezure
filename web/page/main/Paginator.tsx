import * as React from 'react';

import { PostsResponse } from '../../func/api';
import { Link } from '@reach/router';

export type Props = {
    response: PostsResponse | null,
}

export function Paginator(props: Props) {
    const response = props.response;

    function renderPrevPageLink() {
        if (response == null) {
            return null;
        }
        if (response.next == null) {
            return <span className="last-page-marker">ここが最古のページです</span>;
        }
        return <Link to={`/?from=${response.next}`}>←もっと古いの</Link>;
    }

    return <div>
        {renderPrevPageLink()}
    </div>;
}