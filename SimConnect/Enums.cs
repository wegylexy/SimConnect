using System;

namespace FlyByWireless.SimConnect
{
    public enum GroupPriority : uint
    {
        Highest = 1,
        HighestMaskable = 10000000,
        Standard = 1900000000,
        Default = 2000000000,
        Lowest = 4000000000
    }

    public enum RecvId
    {
        Null,
        Exception,
        Open,
        Quit,
        Event,
        EventObjectAddRemove,
        EventFileName,
        EventFrame,
        SimObjectData,
        SimObjectDataByType,
        WeatherObservation,
        CloudState,
        AssignedObjectId,
        ReservedKey,
        CustomAction,
        SystemState,
        ClientData,
        [Obsolete] EventWeatherMode,
        AirportList,
        VORList,
        NDBList,
        WaypointList,
        EventMultiplayerServerStarted,
        EventMultiplayerClientStarted,
        EventMultiplayerSessionEnded,
        EventRaceEnd,
        EventRaceLap
    }

    public enum DataType
    {
        Invalid,
        Int32,
        Int64,
        Float32,
        Float64,
        String8,
        String32,
        String64,
        String128,
        String256,
        String260,
        StringV,
        InitPosition,
        MarkerState,
        Waypoint,
        LatLonAlt,
        XYZ
    }

    public enum Exception
    {
        None,
        Error,
        SizeMismatch,
        UnrecognizedId,
        Unopened,
        VersionMismatch,
        TooManyGroups,
        NameUnrecognized,
        TooManyEventNames,
        EventIdDuplicate,
        TooManyMaps,
        TooManyObjects,
        TooManyRequests,
        WeatherInvalidPort,
        WeatherInvalidMetar,
        WeatherUnableToGetObservation,
        WeatherUnableToCreateStation,
        WeatherUnableToRemoveStation,
        InvalidDataType,
        InvalidDataSize,
        DataError,
        InvalidArray,
        CreateObjectFailed,
        LoadFlightPlanFailed,
        OperationInvalidForObjectType,
        IllegalOperation,
        AlreadySubscribed,
        InvalidEnum,
        DefinitionError,
        DuplicateId,
        DatumId,
        OutOfBounds,
        AlreadyCreated,
        ObjectOutsideRealityBubble,
        ObjectContainer,
        ObjectAi,
        ObjectAtc,
        ObjectSchedule
    }

    public enum SimObjectType
    {
        User,
        All,
        Aircraft,
        Helicopter,
        Boat,
        Ground
    }

    public enum State
    {
        Off,
        On
    }

    public enum Period
    {
        Never,
        Once,
        VisualFrame,
        SimFrame,
        Second
    }

    public enum MissionEnd
    {
        Failed,
        Crashed,
        Succeeded
    }

    public enum ClientDataPeriod
    {
        Never,
        Once,
        VisualFrame,
        OnSet,
        Second
    }

    public enum TextType
    {
        ScrollBlack,
        ScrollWhite,
        ScrollRed,
        ScrollGreen,
        ScrollBlue,
        ScrollYellow,
        ScrollMagenta,
        ScrollCyan,
        PrintBlack = 0x100,
        PrintWhite,
        PrintRed,
        PrintGreen,
        PrintBlue,
        PrintYellow,
        PrintMagenta,
        PrintCyan,
        Menu = 0x200
    }

    public enum TextResult
    {
        MenuSelect1,
        MenuSelect2,
        MenuSelect3,
        MenuSelect4,
        MenuSelect5,
        MenuSelect6,
        MenuSelect7,
        MenuSelect8,
        MenuSelect9,
        MenuSelect10,
        Displayed = 0x10000,
        Queued,
        Removed,
        Replaced,
        Timeout
    }

    public enum WeatherMode
    {
        Theme,
        RWW,
        Custom,
        Global
    }

    public enum FacilityListType
    {
        Airport,
        Waypoint,
        NDB,
        VOR
    }

    [Flags]
    public enum VORFlags
    {
        HasNavSignal = 1,
        HasLocalizer = 2,
        HasGlideSlope = 4,
        HasDME = 8
    }

    [Flags]
    public enum WaypointFlags
    {
        None = 0,
        SpeedRequested = 4,
        ThrottleRequested = 8,
        ComputeVerticalSpeed = 16,
        AltitudeIsAGL = 32,
        OnGround = 0x100000,
        Reverse = 0x200000,
        WrapToFirst = 0x400000
    }

    [Flags]
    public enum EventFlags
    {
        Default = 0,
        FastRepeatTimer = 1,
        SlowRepeatTimer = 2,
        GroupIdIsPriority = 16
    }

    [Flags]
    public enum DataRequestFlags
    {
        Default = 0,
        Changed = 1,
        Tagged = 2
    }

    [Flags]
    public enum DataSetFlags
    {
        Default = 0,
        Tagged = 1
    }

    [Flags]
    public enum ClientDataFlags
    {
        Default = 0,
        ReadOnly = 1
    }

    [Flags]
    public enum ClientDataRequestFlags
    {
        Default = 0,
        Changed = 1,
        Tagged = 2
    }

    [Flags]
    public enum ClientDataSetFlags
    {
        Default = 0,
        Tagged = 1
    }

    [Flags]
    public enum ViewSystemEventDataFlags
    {
        Cockpit2D = 1,
        CockpitVirtual = 2,
        Orthogonal = 4
    }

    [Flags]
    public enum SoundSystemEventDataFlags
    {
        Master = 1
    }
}