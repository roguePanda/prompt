#![allow(non_camel_case_types)]

use std::kinds::marker;
use super::libc::{c_void, c_char, c_int, c_uint, size_t, uint32_t, uint16_t};

// https://github.com/rust-lang/rust/issues/17056
use super::libgit2::{git_repository, git_tree, git_submodule_ignore_t, git_strarray, git_diff_file, git_off_t};
use super::git2::{Repository, Tree, Error};

pub enum git_diff {}

#[allow(dead_code)]
pub enum git_delta_t {
	GIT_DELTA_UNMODIFIED = 0,
	GIT_DELTA_ADDED = 1,
	GIT_DELTA_DELETED = 2,
	GIT_DELTA_MODIFIED = 3,
	GIT_DELTA_RENAMED = 4,
	GIT_DELTA_COPIED = 5,
	GIT_DELTA_IGNORED = 6,
	GIT_DELTA_UNTRACKED = 7,
	GIT_DELTA_TYPECHANGE = 8,
	GIT_DELTA_UNREADABLE = 9,
}

pub type git_diff_notify_cb = extern fn(*const git_diff, *const git_diff_delta, *const c_char, *mut c_void) -> c_int;

#[repr(C)]
pub struct git_diff_options {
	pub version: c_uint,
	pub flags: uint32_t,
	pub ignore_submodules: git_submodule_ignore_t,
	pub pathspec: git_strarray,
	pub notify_cb: Option<git_diff_notify_cb>,
	pub notify_payload: *mut c_void,
	pub context_lines: uint16_t,
	pub interhunk_lines: uint16_t,
	pub id_abbrev: uint16_t,
	pub max_size: git_off_t,
	pub old_prefix: *const c_char,
	pub new_prefix: *const c_char
}

#[repr(C)]
pub struct git_diff_delta {
	pub status: git_delta_t,
	pub flags: uint32_t,
	pub similarity: uint16_t,
	pub nfiles: uint16_t,
	pub old_file: git_diff_file,
	pub new_file: git_diff_file
}

extern {
	fn git_diff_tree_to_workdir_with_index(diff: *mut *mut git_diff, repo: *mut git_repository, old_tree: *mut git_tree, opts: *const git_diff_options) -> c_int;
	fn git_diff_free(diff: *mut git_diff);

	fn git_diff_num_deltas(diff: *const git_diff) -> size_t;
}

pub struct Diff {
	raw: *mut git_diff,
	marker1: marker::NoSend,
	marker2: marker::NoSync
}

impl Diff {
	pub unsafe fn from_raw(raw: *mut git_diff) -> Diff {
		Diff {
			raw: raw,
			marker1: marker::NoSend,
			marker2: marker::NoSync,
		}
	}

	pub fn num_deltas(&self) -> uint {
		unsafe {
			git_diff_num_deltas(self.raw as *const git_diff) as uint
		}
	}
}

#[unsafe_destructor]
impl Drop for Diff {
	fn drop(&mut self) {
		unsafe { git_diff_free(self.raw); }
	}
}

pub fn diff_tree_to_workdir_with_index(repo: &Repository, tree: &Tree) -> Result<Diff, Error> {
	let mut raw_diff = 0 as *mut git_diff;
	unsafe {
		let rc = git_diff_tree_to_workdir_with_index(&mut raw_diff, repo.raw(), tree.raw(), 0 as *const git_diff_options);
		if rc < 0 {
			return Err(Error::last_error().unwrap_or_else(|| {
				Error::from_str("an unknown error occurred")
			}));
		}
		else {
			Ok(Diff::from_raw(raw_diff))
		}
	}
}

pub fn diff_head_to_workdir(repo: &Repository) -> Result<Diff, Error> {
	let head_oid = try!(repo.refname_to_id("HEAD"));
	let head_commit = try!(repo.find_commit(head_oid));
	let head_tree = try!(repo.find_tree(head_commit.tree_id()));
	diff_tree_to_workdir_with_index(repo, &head_tree)
}
