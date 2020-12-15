//! ## BookmarksClient
//!
//! `bookmarks_client` is the module which provides an API between the Bookmarks module and the system

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

// Local
use crate::bookmarks::serializer::BookmarkSerializer;
use crate::bookmarks::{Bookmark, SerializerError, SerializerErrorKind, UserHosts};
use crate::filetransfer::FileTransferProtocol;
// Ext
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

/// ## BookmarksClient
///
/// BookmarksClient provides a layer between the host system and the bookmarks module
pub struct BookmarksClient {
    pub hosts: UserHosts,
    bookmarks_file: PathBuf,
    key_file: PathBuf,
}

impl BookmarksClient {
    /// ### BookmarksClient
    ///
    /// Instantiates a new BookmarksClient
    /// Bookmarks file path must be provided
    /// Key file must be provided
    pub fn new(bookmarks_file: &Path, key_file: &Path) -> Result<BookmarksClient, SerializerError> {
        // Create default hosts
        let default_hosts: UserHosts = Default::default();
        let client: BookmarksClient = BookmarksClient {
            hosts: default_hosts,
            bookmarks_file: PathBuf::from(bookmarks_file),
            key_file: PathBuf::from(key_file),
        };
        // If bookmark file doesn't exist, initialize it
        if !bookmarks_file.exists() {
            if let Err(err) = client.write_bookmarks() {
                return Err(err)
            }
        }
        // If key file doesn't exist, create key
        if !key_file.exists() {
            if let Err(err) = client.generate_key() {
                return Err(err)
            }
        }
        Ok(client)
    }

    /// ### make_bookmark
    /// 
    /// Make bookmark from credentials
    pub fn make_bookmark(&self, addr: String, port: u16, protocol: FileTransferProtocol, username: String, password: Option<String>) -> Bookmark {
        // TODO: crypt password
    }

    /// ### write_bookmarks
    ///
    /// Write bookmarks to file
    pub fn write_bookmarks(&self) -> Result<(), SerializerError> {
        // Open file
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.bookmarks_file.as_path())
        {
            Ok(writer) => {
                let serializer: BookmarkSerializer = BookmarkSerializer {};
                serializer.serialize(Box::new(writer), &self.hosts)
            }
            Err(err) => Err(SerializerError::new_ex(
                SerializerErrorKind::IoError,
                err.to_string(),
            )),
        }
    }

    /// ### generate_key
    /// 
    /// Generate a new AES key and write it to key file
    fn generate_key(&self) -> Result<(), SerializerError> {
        // TODO: write
    }
}
