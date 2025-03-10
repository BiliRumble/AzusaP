import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getSearchResult } from '../../apis/search';
import notFoundImg from '../../assets/images/nodata.png';
import Card from '../../components/numerator/Card';
import SongList from '../../components/organisms/SongList';
import { SearchResult, searchType } from '../../models/search';
import { Song } from '../../models/song';
import styles from './Search.module.scss';

const search = () => {
	const [searchResult, setSearchResult] = useState<SearchResult['result'] | null>(null);
	const [searchType, setSearchType] = useState<searchType>('song');
	const { keyword } = useParams<{ keyword: string }>();
	const navigate = useNavigate();

	useEffect(() => {
		if (!keyword) return navigate('/');
		getSearchResult(keyword, searchType).then((data) => {
			setSearchResult(data);
		});
	}, []);

	useEffect(() => {
		if (!keyword) return navigate('/');
		getSearchResult(keyword, searchType).then((data) => {
			setSearchResult(data);
		});
	}, [keyword, searchType]);

	const renderResults = () => {
		const none = (
			<div className={styles.centBox}>
				<img src={notFoundImg} alt="No data" />
				<h3>没有找到相关内容</h3>
			</div>
		);

		switch (searchType) {
			case 'song':
				if (searchResult && (searchResult.songCount || 0) > 0)
					return <SongList songs={searchResult.songs as unknown as Song[]} />;
				return none;
			case 'playlist':
				if (searchResult && (searchResult.playlistCount || 0) > 0)
					return (
						<div className={styles.search__result__playlist}>
							{searchResult.playlists?.map((playlist) => (
								<Card
									key={playlist.id}
									cover={playlist.coverImgUrl}
									text={playlist.name}
									className={styles.search__result__playlist__card}
									onClick={() => navigate(`/playlist/${playlist.id}`)}
								/>
							))}
						</div>
					);
				return none;
			case 'album':
				if (searchResult && (searchResult.albumCount || 0) > 0)
					return (
						<div className={styles.search__result__album}>
							{searchResult.albums?.map((album) => (
								<Card
									key={album.id}
									cover={album.picUrl}
									text={album.name}
									className={styles.search__result__album__card}
									onClick={() => navigate(`/album/${album.id}`)}
								/>
							))}
						</div>
					);
				return none;
			case 'artist':
				if (searchResult && (searchResult.artistCount || 0) > 0)
					return searchResult.artists?.map((artist) => (
						<div key={artist.id}>{artist.name}</div>
					));
				return none;
			default:
				return none;
		}
	};

	return (
		<div className={styles.search}>
			<div className={styles.search__topbar}>
				<h1>{keyword} 的搜索结果</h1>
				<div className={styles.search__topbar__typeTab}>
					<button
						className={
							styles.search__topbar__typeTab__item +
							' ' +
							(searchType === 'song'
								? styles.search__topbar__typeTab__item__focus
								: '')
						}
						onClick={() => setSearchType('song')}
						disabled={searchType === 'song'}
					>
						歌曲
					</button>
					<button
						className={
							styles.search__topbar__typeTab__item +
							' ' +
							(searchType === 'playlist'
								? styles.search__topbar__typeTab__item__focus
								: '')
						}
						onClick={() => setSearchType('playlist')}
					>
						歌单
					</button>
					<button
						className={
							styles.search__topbar__typeTab__item +
							' ' +
							(searchType === 'album'
								? styles.search__topbar__typeTab__item__focus
								: '')
						}
						onClick={() => setSearchType('album')}
					>
						专辑
					</button>
					<button
						className={
							styles.search__topbar__typeTab__item +
							' ' +
							(searchType === 'artist'
								? styles.search__topbar__typeTab__item__focus
								: '')
						}
						onClick={() => setSearchType('artist')}
					>
						歌手
					</button>
				</div>
			</div>
			<div className={styles.search__result}>{renderResults()}</div>
		</div>
	);
};

export default search;
