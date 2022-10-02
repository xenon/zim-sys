#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

// entry_index_type = u32
// cluster_index_type = u32
// blob_index_type = u32
// size_type = u64
// offset_type = u64
use cxx::{CxxString, CxxVector};
#[cxx::bridge]
pub(crate) mod ffi {

    #[namespace = "zim"]
    extern "C++" {
        pub(crate) type Archive;
        pub(crate) type Blob;
        pub(crate) type Entry;
        pub(crate) type Item;

        pub(crate) type Searcher;
        pub(crate) type Query;
        pub(crate) type Search;
        pub(crate) type SearchResultSet;
        pub(crate) type SearchIterator;

        pub(crate) type SuggestionSearcher;
        pub(crate) type SuggestionSearch;
        pub(crate) type SuggestionResultSet;
        pub(crate) type SuggestionIterator;
        pub(crate) type SuggestionItem;
    }

    unsafe extern "C++" {
        include!("zim-sys/zim-bind.h");

        // FILE: archive.h
        pub(crate) fn archive_ctor_file(path: &str) -> UniquePtr<Archive>;
        pub(crate) fn archive_getFilename(archive: &Archive) -> &CxxString;
        pub(crate) fn archive_getFilesize(archive: &Archive) -> u64;
        pub(crate) fn archive_getAllEntryCount(archive: &Archive) -> u32;
        pub(crate) fn archive_getEntryCount(archive: &Archive) -> u32;
        pub(crate) fn archive_getArticleCount(archive: &Archive) -> u32;
        pub(crate) fn archive_getMetadata(archive: &Archive, name: &str) -> UniquePtr<CxxString>;
        pub(crate) fn archive_getMetadataItem(archive: &Archive, name: &str) -> UniquePtr<Item>;
        pub(crate) fn archive_getMetadataKeys(archive: &Archive)
            -> UniquePtr<CxxVector<CxxString>>;
        pub(crate) fn archive_getEntryByPath_idx(archive: &Archive, idx: u32) -> UniquePtr<Entry>;
        pub(crate) fn archive_getEntryByPath_str(archive: &Archive, path: &str)
            -> UniquePtr<Entry>;
        pub(crate) fn archive_getEntryByTitle_idx(archive: &Archive, idx: u32) -> UniquePtr<Entry>;
        pub(crate) fn archive_getEntryByTitle_str(
            archive: &Archive,
            title: &str,
        ) -> UniquePtr<Entry>;
        pub(crate) fn archive_getMainEntry(archive: &Archive) -> UniquePtr<Entry>;
        pub(crate) fn archive_getRandomEntry(archive: &Archive) -> UniquePtr<Entry>;
        pub(crate) fn archive_hasEntryByPath(archive: &Archive, path: &str) -> bool;
        pub(crate) fn archive_hasEntryByTitle(archive: &Archive, title: &str) -> bool;
        pub(crate) fn archive_hasMainEntry(archive: &Archive) -> bool;
        pub(crate) fn archive_hasFulltextIndex(archive: &Archive) -> bool;
        pub(crate) fn archive_hasTitleIndex(archive: &Archive) -> bool;
        pub(crate) fn archive_hasChecksum(archive: &Archive) -> bool;
        pub(crate) fn archive_getChecksum(archive: &Archive) -> UniquePtr<CxxString>;
        pub(crate) fn archive_check(archive: &Archive) -> bool;
        pub(crate) fn archive_isMultiPart(archive: &Archive) -> bool;
        pub(crate) fn archive_hasNewNamespaceScheme(archive: &Archive) -> bool;

        // FILE: blob.h
        pub(crate) fn blob_ctor() -> UniquePtr<Blob>;
        pub(crate) unsafe fn blob_data(blob: &Blob) -> *const c_char;
        pub(crate) fn blob_size(blob: &Blob) -> u64;

        // FILE: entry.h
        pub(crate) fn entry_isRedirect(entry: &Entry) -> bool;
        pub(crate) fn entry_getTitle(entry: &Entry) -> UniquePtr<CxxString>;
        pub(crate) fn entry_getPath(entry: &Entry) -> UniquePtr<CxxString>;
        pub(crate) fn entry_getItem(entry: &Entry, follow: bool) -> UniquePtr<Item>;
        pub(crate) fn entry_getRedirect(entry: &Entry) -> UniquePtr<Item>;
        pub(crate) fn entry_getRedirectEntry(entry: &Entry) -> UniquePtr<Entry>;
        //pub(crate) fn entry_getRedirectEntryIndex(entry: &Entry) -> u32;
        pub(crate) fn entry_getIndex(entry: &Entry) -> u32;

        // FILE: item.h
        pub(crate) fn item_getTitle(item: &Item) -> UniquePtr<CxxString>;
        pub(crate) fn item_getPath(item: &Item) -> UniquePtr<CxxString>;
        pub(crate) fn item_getMimetype(item: &Item) -> UniquePtr<CxxString>;
        pub(crate) fn item_getData(item: &Item) -> UniquePtr<Blob>;
        pub(crate) fn item_getData_offset(item: &Item, offset: u64, size: u64) -> UniquePtr<Blob>;
        pub(crate) fn item_getSize(item: &Item) -> u64;
        pub(crate) fn item_getIndex(item: &Item) -> u32;

        // FILE: search.h
        pub(crate) fn searcher_ctor(archive: &Archive) -> UniquePtr<Searcher>;
        pub(crate) fn searcher_addArchive(searcher: Pin<&mut Searcher>, archive: &Archive);
        pub(crate) fn searcher_search(
            searcher: Pin<&mut Searcher>,
            query: &Query,
        ) -> UniquePtr<Search>;
        pub(crate) fn searcher_setVerbose(searcher: Pin<&mut Searcher>, verbose: bool);

        pub(crate) fn query_ctor(query: &str) -> UniquePtr<Query>;
        pub(crate) fn query_setQuery(query: Pin<&mut Query>, query_str: &str);
        pub(crate) fn query_setGeorange(
            query: Pin<&mut Query>,
            latitude: f32,
            longitude: f32,
            distance: f32,
        );

        pub(crate) fn search_getResults(
            search: &Search,
            start: i32,
            maxResults: i32,
        ) -> UniquePtr<SearchResultSet>;
        pub(crate) fn search_getEstimatedMatches(search: &Search) -> i32;

        pub(crate) fn searchresultset_begin(
            searchresultset: &SearchResultSet,
        ) -> UniquePtr<SearchIterator>;
        pub(crate) fn searchresultset_end(
            searchresultset: &SearchResultSet,
        ) -> UniquePtr<SearchIterator>;
        pub(crate) fn searchresultset_size(searchresultset: &SearchResultSet) -> i32;

        // FILE: search_iterator.h
        pub(crate) fn searchiterator_operator_eq(
            searchiterator: &SearchIterator,
            o: &SearchIterator,
        ) -> bool;
        pub(crate) fn searchiterator_operator_neq(
            searchiterator: &SearchIterator,
            o: &SearchIterator,
        ) -> bool;
        pub(crate) fn searchiterator_operator_inc(searchiterator: Pin<&mut SearchIterator>);
        pub(crate) fn searchiterator_operator_star(
            searchiterator: &SearchIterator,
        ) -> UniquePtr<Entry>;

        // FILE: suggestion.h
        pub(crate) fn suggestionsearcher_ctor(archive: &Archive) -> UniquePtr<SuggestionSearcher>;
        pub(crate) fn suggestionsearcher_suggest(
            suggestionsearcher: Pin<&mut SuggestionSearcher>,
            query: &str,
        ) -> UniquePtr<SuggestionSearch>;
        pub(crate) fn suggestionsearcher_setVerbose(
            suggestionsearcher: Pin<&mut SuggestionSearcher>,
            verbose: bool,
        );

        pub(crate) fn suggestionsearch_getResults(
            suggestionsearch: &SuggestionSearch,
            start: i32,
            maxResults: i32,
        ) -> UniquePtr<SuggestionResultSet>;
        pub(crate) fn suggestionsearch_getEstimatedMatches(
            suggestionsearch: &SuggestionSearch,
        ) -> i32;

        pub(crate) fn suggestionresultset_begin(
            suggestionresultset: &SuggestionResultSet,
        ) -> UniquePtr<SuggestionIterator>;
        pub(crate) fn suggestionresultset_end(
            suggestionresultset: &SuggestionResultSet,
        ) -> UniquePtr<SuggestionIterator>;
        pub(crate) fn suggestionresultset_size(suggestionresultset: &SuggestionResultSet) -> i32;

        // FILE: suggestion_iterator.h

        pub(crate) fn suggestioniterator_operator_eq(
            suggestioniterator: &SuggestionIterator,
            o: &SuggestionIterator,
        ) -> bool;
        pub(crate) fn suggestioniterator_operator_neq(
            suggestioniterator: &SuggestionIterator,
            o: &SuggestionIterator,
        ) -> bool;
        pub(crate) fn suggestioniterator_operator_inc(
            suggestioniterator: Pin<&mut SuggestionIterator>,
        );
        pub(crate) fn suggestioniterator_operator_star(
            suggestioniterator: Pin<&mut SuggestionIterator>,
        ) -> UniquePtr<SuggestionItem>;
        pub(crate) fn suggestioniterator_getEntry(
            suggestioniterator: &SuggestionIterator,
        ) -> UniquePtr<Entry>;
    }
}

#[cfg(test)]
mod test {
    use std::{pin::Pin, ptr::null};

    use super::*;

    pub static wikt: &str = "/home/aka/Downloads/wiktionary_en_all_maxi_2022-09.zim";

    /// Open a non-empty archive and test archive functions which should work
    #[test]
    fn archive_open_everything() {
        println!("Archive Test:");
        let archive = ffi::archive_ctor_file(wikt);
        assert!(!archive.is_null());
        let archive = archive.as_ref().unwrap();

        let size = ffi::archive_getFilesize(archive);
        println!("  Size: {}", size);
        assert!(size != 0);

        let file = ffi::archive_getFilename(archive);
        println!("  File: {}", file.to_string());

        let all_entry_count = ffi::archive_getAllEntryCount(archive);
        println!("  All Entry Count: {}", all_entry_count);

        let entry_count = ffi::archive_getEntryCount(archive);
        println!("  Entry Count: {}", entry_count);

        let article_count = ffi::archive_getArticleCount(archive);
        println!("  Article Count: {}", article_count);

        let metadata_keys = ffi::archive_getMetadataKeys(archive);
        assert!(!metadata_keys.is_null());
        print!("    Metadata Keys: ");
        for key in metadata_keys.as_ref().unwrap().iter() {
            print!("{} ", key);
        }
        println!("");
    }

    /// Try to open archive that doesn't exist
    #[test]
    fn archive_open_failure() {
        let archive = ffi::archive_ctor_file("////broken..////path///");
        assert!(archive.is_null());
    }

    // Try some invalid metadata queries
    #[test]
    fn archive_metadata_failure() {
        let archive = ffi::archive_ctor_file(wikt);
        assert!(!archive.is_null());
        let archive = archive.as_ref().unwrap();

        let metadata = ffi::archive_getMetadata(archive, "ThisShouldFail!!!");
        assert!(metadata.is_null());

        let metadata2 = ffi::archive_getMetadataItem(archive, "ThisAlsoShouldFail!!!");
        assert!(metadata2.is_null());
    }

    // Try some invalid entry lookups
    #[test]
    fn archive_get_entry_failure() {
        let archive = ffi::archive_ctor_file(wikt);
        assert!(!archive.is_null());
        let archive = archive.as_ref().unwrap();

        let entry =
            ffi::archive_getEntryByPath_str(archive, "ABC/DEF/brokenbrokenborkenSHOULDNTEXIST");
        assert!(entry.is_null());

        let entry2 = ffi::archive_getEntryByTitle_str(archive, "brokenbrokenborkenSHOULDNTEXIST");
        assert!(entry2.is_null());
    }

    // Trivial blob test, create and empty blob and check it's data ptr
    #[test]
    fn blob_trivial() {
        let blob = ffi::blob_ctor();
        assert!(!blob.is_null());
        let blob = blob.as_ref().unwrap();

        unsafe {
            let data = ffi::blob_data(blob);
            assert!(data == null());
        }

        let size = ffi::blob_size(blob);
        assert!(size == 0);
    }

    // Entry test using the main page
    #[test]
    fn entry_mainpage() {
        let archive = ffi::archive_ctor_file(wikt);
        assert!(!archive.is_null());
        let archive = archive.as_ref().unwrap();

        println!("Main page: ");
        let page = ffi::archive_getMainEntry(archive);
        assert!(!page.is_null());
        let page = page.as_ref().unwrap();

        let is_redirect = ffi::entry_isRedirect(page);
        assert!(!is_redirect);

        let title = ffi::entry_getTitle(page);
        assert!(!title.is_null());
        println!("  Title: {}", title.as_ref().unwrap());
    }

    // Try to open the main page item and get some information about it
    #[test]
    fn item_mainpage() {
        let archive = ffi::archive_ctor_file(wikt);
        assert!(!archive.is_null());
        let archive = archive.as_ref().unwrap();

        let page = ffi::archive_getMainEntry(archive);
        assert!(!page.is_null());
        let page = page.as_ref().unwrap();

        let item = ffi::entry_getItem(page, false);
        assert!(!item.is_null());
        let item = item.as_ref().unwrap();

        println!("Main page item: ");
        // finally can do item tests here
        let title = ffi::item_getTitle(item);
        assert!(!title.is_null());
        println!("  Title: {}", title.as_ref().unwrap());

        let path = ffi::item_getPath(item);
        assert!(!path.is_null());
        println!("  Path: {}", path.as_ref().unwrap());

        let mimetype = ffi::item_getMimetype(item);
        assert!(!mimetype.is_null());
        println!("  Mimetype: {}", mimetype.as_ref().unwrap());

        let size = ffi::item_getSize(item);
        println!("  Size: {}", size);

        let index = ffi::item_getIndex(item);
        println!("  Index: {}", index);
    }

    // Try to search for 'name' in wiktionary zim
    #[test]
    fn search_test() {
        let archive = ffi::archive_ctor_file(wikt);
        assert!(!archive.is_null());
        let archive = archive.as_ref().unwrap();

        let mut searcher = ffi::searcher_ctor(archive);
        assert!(!searcher.is_null());
        let mut searcher = searcher.as_mut().unwrap();

        let query = ffi::query_ctor("name");
        assert!(!query.is_null());
        let query = query.as_ref().unwrap();

        let search = ffi::searcher_search(searcher, query);
        assert!(!search.is_null());
        let search = search.as_ref().unwrap();

        let results = ffi::search_getResults(search, 0, 1);
        assert!(!results.is_null());
        let results = results.as_ref().unwrap();

        let size = ffi::searchresultset_size(results);
        assert!(size == 1);
    }

    // Try to search suggestions for 'name' in wiktionary zim
    #[test]
    fn suggestion_test() {
        let archive = ffi::archive_ctor_file(wikt);
        assert!(!archive.is_null());
        let archive = archive.as_ref().unwrap();

        let mut searcher = ffi::suggestionsearcher_ctor(archive);
        assert!(!searcher.is_null());
        let mut searcher = searcher.as_mut().unwrap();

        let search = ffi::suggestionsearcher_suggest(searcher, "name");
        assert!(!search.is_null());
        let search = search.as_ref().unwrap();

        let results = ffi::suggestionsearch_getResults(search, 0, 1);
        assert!(!results.is_null());
        let results = results.as_ref().unwrap();

        let size = ffi::suggestionresultset_size(results);
        assert!(size == 1);
    }
}
