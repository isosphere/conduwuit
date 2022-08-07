pub trait Data {
    fn last_timeline_count(&self, sender_user: &UserId, room_id: &RoomId) -> Result<u64>;

    /// Returns the `count` of this pdu's id.
    fn get_pdu_count(&self, event_id: &EventId) -> Result<Option<u64>>;

    /// Returns the json of a pdu.
    pub fn get_pdu_json(&self, event_id: &EventId) -> Result<Option<CanonicalJsonObject>>;

    /// Returns the json of a pdu.
    pub fn get_non_outlier_pdu_json(

    /// Returns the pdu's id.
    pub fn get_pdu_id(&self, event_id: &EventId) -> Result<Option<Vec<u8>>>;

    /// Returns the pdu.
    ///
    /// Checks the `eventid_outlierpdu` Tree if not found in the timeline.
    pub fn get_non_outlier_pdu(&self, event_id: &EventId) -> Result<Option<PduEvent>>;

    /// Returns the pdu.
    ///
    /// Checks the `eventid_outlierpdu` Tree if not found in the timeline.
    pub fn get_pdu(&self, event_id: &EventId) -> Result<Option<Arc<PduEvent>>>;

    /// Returns the pdu.
    ///
    /// This does __NOT__ check the outliers `Tree`.
    pub fn get_pdu_from_id(&self, pdu_id: &[u8]) -> Result<Option<PduEvent>>;

    /// Returns the pdu as a `BTreeMap<String, CanonicalJsonValue>`.
    pub fn get_pdu_json_from_id(&self, pdu_id: &[u8]) -> Result<Option<CanonicalJsonObject>>;

    /// Returns the `count` of this pdu's id.
    pub fn pdu_count(&self, pdu_id: &[u8]) -> Result<u64>;

    /// Removes a pdu and creates a new one with the same id.
    fn replace_pdu(&self, pdu_id: &[u8], pdu: &PduEvent) -> Result<()>;

    /// Returns an iterator over all events in a room that happened after the event with id `since`
    /// in chronological order.
    #[tracing::instrument(skip(self))]
    pub fn pdus_since<'a>(
        &'a self,
        user_id: &UserId,
        room_id: &RoomId,
        since: u64,
    ) -> Result<impl Iterator<Item = Result<(Vec<u8>, PduEvent)>> + 'a>;

    /// Returns an iterator over all events and their tokens in a room that happened before the
    /// event with id `until` in reverse-chronological order.
    #[tracing::instrument(skip(self))]
    pub fn pdus_until<'a>(
        &'a self,
        user_id: &UserId,
        room_id: &RoomId,
        until: u64,
    ) -> Result<impl Iterator<Item = Result<(Vec<u8>, PduEvent)>> + 'a>;

    pub fn pdus_after<'a>(
        &'a self,
        user_id: &UserId,
        room_id: &RoomId,
        from: u64,
    ) -> Result<impl Iterator<Item = Result<(Vec<u8>, PduEvent)>> + 'a>;
}
