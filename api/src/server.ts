import express from "express";
import Jikan from 'jikan4.js';
import { load, type CheerioAPI, type SelectorType } from "cheerio";
import createHttpError from "http-errors";
import axios, { AxiosError } from "axios";

const app = express();
const PORT = process.env.PORT ?? 3001;

type HeaderConfig = {
  "USER_AGENT_HEADER": string,
  "ACCEPT_ENCODEING_HEADER": string,
  "ACCEPT_HEADER": string
}

const headers: HeaderConfig = {
  USER_AGENT_HEADER: "Mozilla/5.0 (X11; Linux x86_64; rv:122.0) Gecko/20100101 Firefox/122.0",
  ACCEPT_ENCODEING_HEADER: "gzip, deflate, br",
  ACCEPT_HEADER: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"
}

type CustomEpisodeType = {
  "id": string | undefined;
  "episode_no": number | undefined;
  "title": string | undefined;
  "is_filler": boolean | undefined;
}

type CustomType = {
  "id": number | undefined;
  "title": string | null;
  "mal_id": string | number | undefined;
  "al_id": number;
  "japanese_title": string;
  "image": string | undefined;
  "description": string | undefined;
  "category": string | undefined;
  "sub_or_dub": string | undefined,
  "total_episodes": number | undefined;
  "episodes": CustomEpisodeType[];
}

async function fetchAbout(id: string) {
  try {
    const URLs = await URL_fn();
    const aboutURL: string = new URL(id, URLs.BASE).toString();
    const mainPage = await axios.get(aboutURL, {
      headers: {
        "User-Agent": headers.USER_AGENT_HEADER,
        "Accept-Encoding": headers.ACCEPT_ENCODEING_HEADER,
        Accept: headers.ACCEPT_HEADER,
      },
    });

    const $: CheerioAPI = load(mainPage.data);
    const selectors: SelectorType = "#ani_detail .container .anis-content";

    const episodes = await axios.get(
      `${URLs.AJAX}/v2/episode/list/${id.split("-").pop()}`,
      {
        headers: {
          "User-Agent": headers.USER_AGENT_HEADER,
          "X-Requested-With": "XMLHttpRequest",
          "Accept-Encoding": headers.ACCEPT_ENCODEING_HEADER,
          Accept: headers.ACCEPT_HEADER,
          Referer: `${URLs.BASE}/watch/${id}`,
        },
      },
    );

    const $e: CheerioAPI = load(episodes.data.html);
    const e_selectors: SelectorType = ".detail-infor-content .ss-list a";

    let aboutData = extract_about_info($, selectors);
    let episodeObj = {
      "episodes": extract_episodes_info($e, e_selectors, aboutData.sub_or_dub)
    }
    return {
      ...aboutData,
      ...extract_extra_about_info($, selectors),
      ...episodeObj
    };

  } catch (err) {
    if (err instanceof AxiosError) {
      throw createHttpError(
        err?.response?.status || 500,
        err?.response?.statusText || "Something went wrong",
      );
    } else {
      throw createHttpError.InternalServerError("Internal server error");
    }
  }
}

app.get("/anime/:anime", async (req, res) => {
  console.log(req.params.anime)
  let response = await fetchAbout(req.params.anime);
  res.status(200).json(response);
});

// app.get("/anime/:anime", (req, res) => {
//   console.log(`/anime/${req.params.anime}`)
// let episodes: CustomEpisodeType[] = []
// let response: CustomType;
//
// zoro.fetchAnimeInfo(req.params.anime).then(data => {
//
//   if (data.episodes) {
//     for (const episode of data.episodes) {
//       let episode_detail: CustomEpisodeType = {
//         id: episode.id,
//         title: episode.title,
//         episode_no: episode.number,
//         is_filler: episode.isFiller,
//       };
//       episodes.push(episode_detail);
//     }
//   }
//
//   response = {
//     id: Number(data.id.split("-").pop()),
//     title: data.title,
//     mal_id: data.malID,
//     al_id: data.alID,
//     japanese_title: data.japaneseTitle,
//     image: data.image,
//     description: data.description,
//     category: data.type,
//     total_episodes: data.totalEpisodes,
//     sub_or_dub: data.subOrDub,
//     episodes: episodes
//   }
//   // console.log(response)
//   res.status(200).json(response);
// })
// })

app.listen(PORT, () => {
  console.log(`⚔️  API started ON PORT : ${PORT} @ STARTED  ⚔️`);
});

interface MinimalAnime {
  id: number | null;
  title: string | null;
  image: string | null;
}

interface Anime extends MinimalAnime {
  total_episodes: number | null;
  sub_episodes: number | null;
  dub_episodes: number | null;
}

interface AboutAnimeInfo extends Anime {
  sub_or_dub: string | null | undefined;
  mal_id: number | null;
  al_id: number | null;
  rating: string | null;
  category: string | null;
  quality: string | null;
  description: string | null;
}

const extract_about_info = (
  $: CheerioAPI,
  selectors: SelectorType,
): AboutAnimeInfo => {
  try {
    let info: AboutAnimeInfo | undefined;

    $(selectors).each((_index, _element) => {
      const { mal_id, anilist_id } = JSON.parse($('#syncData').text());
      const hasSub: boolean = $('div.film-stats div.tick div.tick-item.tick-sub').length > 0;
      const hasDub: boolean = $('div.film-stats div.tick div.tick-item.tick-dub').length > 0;
      const animeID =
        $(selectors)
          .find(".anisc-detail .film-buttons a.btn-play")
          .attr("href")
          ?.split("/")
          ?.pop()?.split("-").pop() || null;
      const animeNAME =
        $(selectors)
          .find(".anisc-detail .film-name.dynamic-name")
          .text()
          .trim() ?? "UNKNOWN ANIME";
      const animeIMG =
        $(selectors)
          .find(".film-poster .film-poster-img")
          ?.attr("src")
          ?.trim() ?? "UNKNOWN";
      const animeRATING =
        $(`${selectors} .film-stats .tick .tick-pg`)?.text()?.trim() || null;
      const animeQUALITY =
        $(`${selectors} .film-stats .tick .tick-quality`)?.text()?.trim() ||
        null;
      const epSUB =
        Number($(`${selectors} .film-stats .tick .tick-sub`)?.text()?.trim()) ||
        null;
      const epDUB =
        Number($(`${selectors} .film-stats .tick .tick-dub`)?.text()?.trim()) ||
        null;
      const total_eps =
        Number($(`${selectors} .film-stats .tick .tick-eps`)?.text()?.trim()) ||
        null;
      const animeCategory =
        $(`${selectors} .film-stats .tick`)
          ?.text()
          ?.trim()
          ?.replace(/[\s\n]+/g, " ")
          ?.split(" ")
          ?.at(-2) || null;
      const duration =
        $(`${selectors} .film-stats .tick`)
          ?.text()
          ?.trim()
          ?.replace(/[\s\n]+/g, " ")
          ?.split(" ")
          ?.pop() || null;
      const animeDESCRIPTION =
        $(selectors)
          .find(".anisc-detail .film-description .text")
          ?.text()
          ?.trim() ?? "UNKNOW ANIME DESCRIPTION";

      let subDub;
      if (hasSub) {
        subDub = 'sub';
      }
      if (hasDub) {
        subDub = 'dub';
      }
      if (hasSub && hasDub) {
        subDub = 'both';
      }

      info = {
        id: Number(animeID),
        mal_id: Number(mal_id),
        al_id: Number(anilist_id),
        title: animeNAME,
        image: animeIMG,
        rating: animeRATING,
        total_episodes: total_eps,
        sub_episodes: epSUB,
        dub_episodes: epDUB,
        category: animeCategory,
        quality: animeQUALITY,
        description: animeDESCRIPTION,
        sub_or_dub: subDub,
      };
    });

    if (info === undefined) {
      info = {
        id: null,
        mal_id: null,
        al_id: null,
        title: null,
        image: null,
        rating: null,
        total_episodes: null,
        sub_episodes: null,
        dub_episodes: null,
        category: null,
        quality: null,
        description: null,
        sub_or_dub: 'sub',
      };
    }

    return info;
  } catch (err) {
    ///////////////////////////////////////////////////////////////////
    console.error("Error in extract_about_info :", err); // for TESTING//
    ///////////////////////////////////////////////////////////////////

    if (err instanceof AxiosError) {
      throw createHttpError(
        err?.response?.status || 500,
        err?.response?.statusText || "Something went wrong",
      );
    } else {
      throw createHttpError.InternalServerError("Internal server error");
    }
  }
};

type ExtraAboutAnimeInfo = Record<string, string | string[]>;

const extract_extra_about_info = (
  $: CheerioAPI,
  selectors: SelectorType,
): ExtraAboutAnimeInfo => {
  try {
    const moreInfo: ExtraAboutAnimeInfo = {};
    const genres: string[] = [];
    const producers: string[] = [];

    $(selectors + " .item-title").each((_index, element) => {
      const animeKEY: string =
        $(element).find(".item-head")?.text()?.trim() ?? "UNKNOWN";
      const animeVALUE = $(element).find(".name")?.text()?.trim() ?? "UNKNOWN";

      if (animeKEY !== "Producers:" && animeKEY !== "Overview:" && animeKEY !== "Japanese:") {
        // TODO: make fn for transformer
        moreInfo[animeKEY.split(":")[0].toLowerCase().replace(" ", "_")] = animeVALUE;
      } else if (animeKEY === "Producers:") {
        $(selectors + " .item-title a").each((_index, element) => {
          const animeProducers = $(element)?.text()?.trim() ?? "UNKNOWN";
          producers.push(animeProducers);
        });
      } else if (animeKEY === "Japanese:") {
        moreInfo["japanese_title"] = animeVALUE;
      }
    });

    $(selectors + " .item-list a").each((_index, element) => {
      const animeGENRES = $(element)?.text()?.trim() ?? "UNKNOWN";
      genres.push(animeGENRES);
    });

    moreInfo["genres"] = genres.join();
    moreInfo["producers"] = producers.join();

    return moreInfo;
  } catch (err) {
    ///////////////////////////////////////////////////////////////////
    console.error("Error in extract_extra_about_info :", err); // for TESTING//
    ///////////////////////////////////////////////////////////////////

    if (err instanceof AxiosError) {
      throw createHttpError(
        err?.response?.status || 500,
        err?.response?.statusText || "Something went wrong",
      );
    } else {
      throw createHttpError.InternalServerError("Internal server error");
    }
  }
};



type AniWatchConfig = {
  BASE: string,
  HOME: string,
  SEARCH: string,
  GENRE: string,
  AJAX: string,
}

type WebsiteConfig = {
  BASE: string,
}

export type AnimeWebsiteConfig = WebsiteConfig & {
  CLONES?: Record<string, string[]>,
}

type Websites = Record<string, AnimeWebsiteConfig>;

// anime websites and their clones
export const websites_collection: Websites = {
  "AniWatch": {
    BASE: "https://aniwatchtv.to",
    CLONES: {
      "HiAnime": ["https://hianime.to", "https://hianime.nz", "https://hianime.sx"],
    }
  },
  "GogoAnime": {
    BASE: "https://gogoanime3.co",
  }
}

const aniwatch: AnimeWebsiteConfig = websites_collection["AniWatch"];
// storing initial base link
let aniwatch_base = aniwatch.BASE;
// array of clones
let clones_array: string[] = [];
clones_array.push(aniwatch_base);

if (aniwatch.CLONES) {
  const aniwatch_clones: Record<string, string[]> = aniwatch.CLONES;

  for (const key in aniwatch_clones) {
    if (Object.prototype.hasOwnProperty.call(aniwatch_clones, key)) {
      const values: string[] = aniwatch_clones[key];
      clones_array.push(...values);
    }
  }
}

// Testing
// console.log(clones_array);

// make new aniwatchobj using new aniwatch_base
const makeAniWatchObj = (aniwatch_base: string): AniWatchConfig => {
  // Testing
  // console.log(aniwatch_base);
  return {
    BASE: aniwatch_base,
    HOME: `${aniwatch_base}/home`,
    SEARCH: `${aniwatch_base}/search`,
    GENRE: `${aniwatch_base}/genre`,
    AJAX: `${aniwatch_base}/ajax`,
  }
}

// return fn
const URL_fn = async (): Promise<AniWatchConfig> => {
  try {
    for (const url of clones_array) {
      if (await isSiteReachable(url as string)) {
        aniwatch_base = url;
        break;
      }
    }
    return makeAniWatchObj(aniwatch_base as string);
  } catch (error) {
    console.error("Error occurred in both sites:", error);
    throw error; // Rethrow the error to handle it outside
  }
};

export const isSiteReachable = async (url: string): Promise<boolean> => {
  try {
    const response = await fetch(url, { method: 'HEAD' });
    return response.ok;
  } catch (error) {
    return false;
  }
}


type Episode = {
  title: string | null;
  episode_no: number | null;
  id: string | null;
  is_filler: boolean | false;
}

export const extract_episodes_info = (
  $: CheerioAPI,
  selectors: SelectorType,
  subOrDub: string | null | undefined
) => {
  try {
    const episodes: Episode[] = [];

    $(`${selectors}`).each((_index, element) => {
      episodes.push({
        title: $(element)?.attr("title")?.trim() || null,
        episode_no: Number($(element).attr("data-number")),
        id: $(element)?.attr('href')
          ?.split('/')[2]
          ?.replace('?ep=', '$episode$')
          ?.concat(`$${subOrDub}`)!,
        is_filler: $(element).hasClass("ssl-item-filler"),
      });
    });

    return episodes;
  } catch (err) {
    ///////////////////////////////////////////////////////////////////
    console.error("Error in extract_episodes_info :", err); // for TESTING//
    ///////////////////////////////////////////////////////////////////

    if (err instanceof AxiosError) {
      throw createHttpError(
        err?.response?.status || 500,
        err?.response?.statusText || "Something went wrong",
      );
    } else {
      throw createHttpError.InternalServerError("Internal server error");
    }
  }
};

const animeClient = new Jikan.Client();

app.get("/anime/:anime_mal_id/staff", async (req, res) => {
  try {
    const animeMalId = Number(req.params.anime_mal_id);
    if (isNaN(animeMalId)) {
      return res.status(400).json({ error: 'Invalid anime_mal_id' });
    }
    const response = await animeClient.anime.getStaff(animeMalId);
    res.status(200).json(response);
  } catch (error) {
    console.error('Error fetching anime staff:', error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

export default app;
