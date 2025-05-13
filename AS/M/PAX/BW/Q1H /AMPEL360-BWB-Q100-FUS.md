The provided YAML file represents a detailed instance data specification for the **AMPEL360 BWB-Q100 (Blended Wing Body aircraft)**, designated as **S/N 00001**, under flight testing. Below are the key highlights and structured breakdowns of the data:

---

### **Core Identification**

- **Instance GAIA ID**: `AS-M-PAX-BW-Q1H-00001`
  - **Domain**: Air System (`AS`)
  - **Autonomy**: Manned/Semi-Autonomous (`M`)
  - **Functional Class**: Passenger Transport (`PAX`)
  - **Subtype**: Blended Wing Body (`BW`)
  - **Model**: `Q1H` (AMPEL360 BWB-Q100)
  - **Serial Number**: `00001`

---

### **Model Information**
- **Model Code**: `Q1H`
  - **Name**: AMPEL360 BWB-Q100
  - **Description**: Small quantum-enhanced BWB passenger aircraft
  - **Specifications**:
    - Capacity: 100 passengers
    - Range: 5,500 km
    - Integrated Quantum Systems:
      - Navigation
      - Optimization
      - Communications
  - **Catalog Notes**: Efficient BWB design with quantum navigation and optimization.

---

### **Instance-Specific Data**

- **Serial Number**: `00001`
- **Role Profile**: Flight Test & Baseline Definition Unit
- **Registration Number**: `EC-QBWFT`
- **Manufacturer**: GAIA Aerospace Consortium
- **Key Dates**:
  - Manufacturing: `2025-01-15`
  - First Flight: `2025-02-28`
- **Current Operator**: GAIA-QAO Test & Evaluation Wing
- **Location**:
  - Facility: GAIA-QAO Flight Test Center
  - Coordinates: 37.3171°N, -5.8070°W
- **Operational Status**: Active (Undergoing Flight Test Program)
- **Airframe Hours**: 75.5
- **Cycles**: 30
- **Instance Notes**: Primary flight test vehicle for performance, safety validation, and baseline configuration certification.

---

### **Configuration Management**
- **Current Configuration Code**: `E1` (Experimental/Test Configuration)
- **Description**: Flight Test Instrumentation Configuration
  - Includes specialized instrumentation and data acquisition systems.
  - Targeting the baseline production configuration (A1) through testing.

---

### **Installed Subsystems**
1. **Propulsion System**:
   - **Subsystem ID**: `PROP-QEP-L001T`
   - **Model**: QuantumFlow Series 1 - Test Version - Left
   - **Position**: Port Engine Mount
   - **Instrumentation**: Extensive (Flight Test)

2. **Flight Control System**:
   - **Subsystem ID**: `FCS-PFCXT-001`
   - **Model**: AdvancedFlightCtrl MkI-XT
   - **Position**: Main Avionics Bay
   - **Instrumentation**: Extensive (Data Logging)

3. **Flight Test Instrumentation Suite**:
   - **Subsystem ID**: `AVIO-FTI-001`
   - **Model**: Comprehensive FTI Package Rev A
   - **Position**: Distributed (Cabin Racks, Avionics Bays)

---

### **Maintenance Summary**
- **Last Check**: Pre-Flight Test Block C.2 Inspection & Calibration (`2025-05-09`)
- **Next Check**: Post-Sortie Inspections (as per flight test plan)

---

### **Document Metadata**
- **Filename**: `AS-M-PAX-BW-Q1H-A53`
- **Version**: `0.9.0`
- **Date**: `2025-05-12`
- **Status**: Review (reflecting ongoing test program)
- **Security**: Confidential
- **Keywords**: Flight Test, Experimental, AMPEL360 BWB-Q100

---

This YAML file provides precise configuration and operational details essential for managing flight testing and documenting the progression of the **AMPEL360 BWB-Q100**.

### Main Fuselage/Central Body Assembly (AS-M-PAX-BW-Q1H-FUS) - Rev C


---

## Conceptual Bill of Materials (BOM) - Main Fuselage/Central Body Assembly (AS-M-PAX-BW-Q1H-FUS) - Rev C (Expandido)

**Aircraft Top-Level Object ID (Tier 1):** `AS-M-PAX-BW-Q1H-00001` *(Illustrative Instance)*
**Main Assembly ID (Tier 2 Conceptual):** `AS-M-PAX-BW-Q1H-00001 :: STR-CBA-D001-S0001`
**Main Assembly Name:** Main Fuselage/Central Body Assembly - BWB-Q100Small
**Date:** YYYY-MM-DD
**Revision:** C

| Level | Item No. | GAIA-QAO Subsystem ID (Conceptual `SSS-MDLs-SERs`) | Part/Assembly Name                         | Qty | Unit | Material (Conceptual)         | Dimensiones (Conceptual) (LxWxH mm) | Peso Espec. (kg/m³ o kg/unidad) | Remarks                                                                                         | Prompt-Train Generación Plano (Conceptual)                                                                                                                               |
| :---- | :------- | :------------------------------------------------ | :----------------------------------------- | :-- | :--- | :---------------------------- | :------------------------------------ | :------------------------------ | :---------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1** | **1.0**  | **`STR-CBA-D001-S0001`**                          | **Main Fuselage/Central Body Assembly**    | **1** | **EA** | **Various (Assembly)**        | 35000x25000x6000 (Envolvente)         | Est. 15000 kg (Total)           | **Top-Level Fuselage Assy.**                                                                    | `1.Load_Master_Assy(STR-CBA-D001)<br>2.Define_GA_Views<br>3.Detail_Major_Interfaces<br>4.Generate_Zone_Diags<br>5.Populate_Title_Block(GA)`                                     |
| 2     | 1.1      | `STR-SHL-D001-S0001`                              | Outer Shell & Aerodynamic Surfaces Assembly| 1   | EA   | Various (Composite Assy)      | 35000x25000x3000 (Envolvente Sup/Inf) | Est. 4000 kg                    | SSS: `STR-SHL` (Structure-Shell). Houses aerodynamic & some system interfaces.                  | `1.Load_Assy(SHL-D001)<br>2.Create_Surf_Continuity_Views<br>3.Detail_Panel_Interfaces<br>4.Generate_Assy_BOM<br>5.Populate_Title_Block(SHL_Assy)`                             |
| 3     | 1.1.1    | `STR-SHL-UPNLD01-SXXXX`                           | Upper Surface Panel Assembly (e.g., Fwd Upper) | 1   | EA   | CFRP T800/M55J                | 8000x4000x50 (Panel típico)         | ~1700 kg/m³                     | Qty varies per specific panel P/N. `MDL: UPNLD01` represents this panel type.                  | `1.Load_Model(UPNLD01)<br>2.Define_Views(Flat_Pattern,Formed,Iso)<br>3.Add_Ply_Schedule_Ref<br>4.Detail_Edge_GD&T<br>5.Populate_Title_Block(UPNLD01_Dwg)`                          |
| 3     | 1.1.2    | `STR-SHL-LPNLD01-SXXXX`                           | Lower Surface Panel Assembly (e.g., Center Lower) | 1   | EA   | CFRP T800/M55J                | 7000x3500x40 (Panel típico)         | ~1700 kg/m³                     | Incl. access door cutouts. `MDL: LPNLD01` for this panel type.                                  | `1.Load_Model(LPNLD01)<br>2.Define_Views_Incl_Cutouts<br>3.Add_Ply_Dropoff_Details<br>4.Detail_Interface_GD&T<br>5.Populate_Title_Block(LPNLD01_Dwg)`                        |
| 3     | 1.1.3    | **`A30-LEDSA-C001-SXXXX`**                        | Leading Edge Sub-Assembly (Central Section) | 1   | EA   | CFRP / Ti                     | 12000x500x300 (Sección Central)     | Est. 300 kg                     | SSS: `A30` (Ice/Rain Prot.) as it includes de-icing. `MDL: LEDSA-C001` (LE De-iced Sub-Assy).   | `1.Load_Assy(LEDSA-C001)<br>2.Detail_Deicing_Sys_Interface<br>3.Show_Attach_Points<br>4.Specify_Surf_Finish<br>5.Populate_Title_Block(LEDSA_Dwg)`                            |
| 3     | 1.1.4    | **`A27-TESA-C001-SXXXX`**                         | Trailing Edge Sub-Assembly (Central Section) | 1   | EA   | CFRP                          | 12000x800x200 (Sección Central)     | Est. 250 kg                     | SSS: `A27` (Flight Controls) as it includes primary control surface interfaces. `MDL: TESA-C001`. | `1.Load_Assy(TESA-C001)<br>2.Detail_Ctrl_Surf_Hinge_Lines<br>3.Show_Actuator_Attach<br>4.Specify_Mat_ layup<br>5.Populate_Title_Block(TESA_Dwg)`                             |
| 3     | 1.1.5    | **`QCO-ANTCONF-A01-SXXXX`**                       | Quantum Communications Conformal Antenna Array Assy | 2   | EA   | Dielectric Composite/RF Mat.  | 1500x800x50 (Por Array)             | ~2000 kg/m³ (Estimado)          | SSS: `QCO` (Quantum Comms). `MDL: ANTCONF-A01`. Integrated into OML.                            | `1.Load_Model(ANTCONF-A01)<br>2.Define_Installation_Views<br>3.Specify_RF_Props_Notes<br>4.Detail_Keepout_Zones<br>5.Populate_Title_Block(ANTCONF_Dwg)`                       |
| 3     | 1.1.6    | `STR-FAIRWBR-P01-SXXXX`                           | Wing-to-Body Fairing Assembly (Port)        | 1   | EA   | CFRP                          | 3000x1000x800 (Aprox.)              | Est. 50 kg                      | SSS: `STR`. `MDL: FAIRWBR-P01`. If not integral to main panels.                                 | `1.Load_Model(FAIRWBR-P01)<br>2.Define_Views_Showing_Blend<br>3.Detail_Attach_Interfaces<br>4.Specify_Surf_Smoothness<br>5.Populate_Title_Block(FAIRWBR-P01_Dwg)`                 |
| 3     | 1.1.7    | `STR-FAIRWBR-S01-SXXXX`                           | Wing-to-Body Fairing Assembly (Starboard)   | 1   | EA   | CFRP                          | 3000x1000x800 (Aprox.)              | Est. 50 kg                      | SSS: `STR`. `MDL: FAIRWBR-S01`. If not integral to main panels.                                 | `1.Load_Model(FAIRWBR-S01)<br>2.Define_Views_Showing_Blend<br>3.Detail_Attach_Interfaces<br>4.Specify_Surf_Smoothness<br>5.Populate_Title_Block(FAIRWBR-S01_Dwg)`                 |
| 2     | 1.2      | `STR-CORESA-D001-S0001`                           | Primary Structural Core Assembly            | 1   | EA   | Various (Composite/Metallic)  | 30000x15000x5000 (Envolvente)      | Est. 6000 kg                    | SSS: `STR-CORESA` (Structure-Core Sub-Assembly). The main load-bearing frame.                 | `1.Load_Assy(CORESA-D001)<br>2.Show_Load_Path_Highlights<br>3.Detail_Major_Joints<br>4.Generate_Internal_BOM<br>5.Populate_Title_Block(CORESA_Assy)`                          |
| 3     | 1.2.1    | `STR-SPARMC-P01-SXXXX`                            | Main Spar Assembly (Center Carry-Through, Port) | 1   | EA   | CFRP / Ti Fittings            | 15000x1200x300 (Aprox.)             | Est. 800 kg                     | SSS: `STR`. `MDL: SPARMC-P01`.                                                                  | `1.Load_Model(SPARMC-P01)<br>2.Define_Section_Views<br>3.Detail_Fitting_Interfaces<br>4.Specify_Fastener_Reqs<br>5.Populate_Title_Block(SPARMC-P01_Dwg)`                       |
| 3     | 1.2.2    | `STR-SPARMC-S01-SXXXX`                            | Main Spar Assembly (Center Carry-Through, Stbd) | 1   | EA   | CFRP / Ti Fittings            | 15000x1200x300 (Aprox.)             | Est. 800 kg                     | SSS: `STR`. `MDL: SPARMC-S01`.                                                                  | `1.Load_Model(SPARMC-S01)<br>2.Define_Section_Views<br>3.Detail_Fitting_Interfaces<br>4.Specify_Fastener_Reqs<br>5.Populate_Title_Block(SPARMC-S01_Dwg)`                       |
| 3     | 1.2.3    | `STR-RIBSET-C01-SXXXX`                            | Rib Assembly Set (Frames FS_XXX to FS_YYY)  | 1   | SET  | CFRP / Al Alloy               | Varies per rib                      | Est. 10-50 kg/rib               | SSS: `STR`. `MDL: RIBSET-C01`. This MDL represents the entire kit/set of ribs.                   | `1.Generate_Sheet_Per_Rib_Type(RIBSET-C01)<br>2.For_Each_Rib:Load_Model,Define_Views,Detail_Cutouts,GD&T<br>3.Create_Kit_List_Sheet<br>4.Populate_Title_Blocks`                  |
| 3     | 1.2.4    | `STR-FRMSET-C01-SXXXX`                            | Frame Assembly Set (Frames STA_AAA to STA_BBB) | 1   | SET  | CFRP / Al Alloy               | Varies per frame                    | Est. 20-100 kg/frame            | SSS: `STR`. `MDL: FRMSET-C01`. This MDL represents the entire kit/set of frames.                  | `1.Generate_Sheet_Per_Frame_Type(FRMSET-C01)<br>2.For_Each_Frame:Load_Model,Define_Views,Detail_Interfaces,GD&T<br>3.Create_Kit_List_Sheet<br>4.Populate_Title_Blocks`            |
| 3     | 1.2.5    | `STR-GRIDST-C01-SXXXX`                            | Stringer & Stiffener Set                    | 1   | SET  | CFRP / Al Alloy               | Varies per member                   | ~1700 kg/m³ (CFRP)              | SSS: `STR`. `MDL: GRIDST-C01`. Distributed across shell.                                        | `1.Generate_Detail_Sheet_Per_Profile(GRIDST-C01)<br>2.For_Each_Profile:Define_XSection,Specify_Length_Cut_List<br>3.Create_Installation_Guide_Ref<br>4.Populate_Title_Blocks` |
| 3     | 1.2.6    | `STR-FBHDPRS-A01-SXXXX`                           | Forward Pressure Bulkhead Assembly          | 1   | EA   | CFRP / Al Alloy Honeycomb     | 5000x3000x200 (Aprox.)              | Est. 350 kg                     | SSS: `STR`. `MDL: FBHDPRS-A01`.                                                                 | `1.Load_Model(FBHDPRS-A01)<br>2.Define_Views_Showing_Curvature<br>3.Detail_Seal_Groove_Interface<br>4.Specify_Pressure_Test_Reqs<br>5.Populate_Title_Block(FBHDPRS-A01_Dwg)`         |
| 3     | 1.2.7    | `STR-ABHDPRS-A01-SXXXX`                           | Aft Pressure Bulkhead Assembly              | 1   | EA   | CFRP / Al Alloy Honeycomb     | 4500x2500x250 (Aprox.)              | Est. 300 kg                     | SSS: `STR`. `MDL: ABHDPRS-A01`.                                                                 | `1.Load_Model(ABHDPRS-A01)<br>2.Define_Views_Showing_Curvature<br>3.Detail_System_Feedthroughs<br>4.Specify_Pressure_Test_Reqs<br>5.Populate_Title_Block(ABHDPRS-A01_Dwg)`         |
| 3     | 1.2.8    | `STR-BHDNPSET-A01-SXXXX`                          | Non-Pressurized Bay Bulkhead Set (MLG, Q-Bays) | 1   | SET  | CFRP / Al Alloy               | Varies per bulkhead                 | Est. 30-80 kg/bulkhead          | SSS: `STR`. `MDL: BHDNPSET-A01`. Divides internal zones.                                        | `1.Generate_Sheet_Per_Bulkhead(BHDNPSET-A01)<br>2.For_Each:Load_Model,Define_Views,Detail_Interfaces,GD&T<br>3.Create_Set_List_Sheet<br>4.Populate_Title_Blocks`                  |
| 3     | 1.2.9    | `STR-KEELBM-A01-SXXXX`                            | Keel Beam Assembly (If applicable)          | 1   | EA   | Ti / CFRP                     | 10000x400x600 (Aprox.)              | Est. 500 kg                     | SSS: `STR`. `MDL: KEELBM-A01`.                                                                  | `1.Load_Model(KEELBM-A01)<br>2.Define_Views_Highlighting_Joints<br>3.Detail_Cross_Sections<br>4.Specify_Material_Transitions<br>5.Populate_Title_Block(KEELBM-A01_Dwg)`           |
| 2     | 1.3      | `STR-CABSTR-D001-S0001`                           | Cabin Section Structural Assembly           | 1   | EA   | Various (Composite/Metallic)  | 20000x8000x2500 (Envolvente Cabina) | Est. 1500 kg                    | SSS: `STR-CABSTR`. Excludes furnishings (`SSS:A25`).                                            | `1.Load_Assy(CABSTR-D001)<br>2.Show_Furnishing_Attach_Provisions<br>3.Detail_Floor_Grid_Layout<br>4.Generate_Internal_BOM<br>5.Populate_Title_Block(CABSTR_Assy)`                   |
| 3     | 1.3.1    | `STR-CKPSTR-A01-SXXXX`                            | Cockpit Module Structure Assembly           | 1   | EA   | Al Alloy / CFRP               | 3000x2500x2000 (Aprox.)             | Est. 200 kg                     | SSS: `STR-CKPSTR`. Windscreen frame, panel supports, seat mounts.                               | `1.Load_Assy(CKPSTR-A01)<br>2.Detail_Instrument_Panel_Mounts<br>3.Show_Seat_Rail_Interfaces<br>4.Specify_Visibility_Zone_Refs<br>5.Populate_Title_Block(CKPSTR_Dwg)`               |
| 4     | 1.3.1.1  | `STR-WFRM-A01-SXXXX`                              | Windscreen Frame                            | 1   | EA   | Ti / Al Alloy                 | 2000x1500x100 (Aprox.)              | Est. 40 kg                      | Part of Cockpit Module Structure. `MDL: WFRM-A01`.                                              | `1.Load_Model(WFRM-A01)<br>2.Define_Views_Showing_Angles<br>3.Detail_Glazing_Interface<br>4.Specify_Bird_Strike_Req_Ref<br>5.Populate_Title_Block(WFRM-A01_Dwg)`                    |
| 3     | 1.3.2    | `STR-PAXSTR-A01-SXXXX`                            | Passenger Cabin Structure Assembly          | 1   | EA   | Al Alloy / CFRP               | 17000x8000x2500 (Aprox.)            | Est. 1200 kg                    | SSS: `STR-PAXSTR`. Floor grid, sidewall/overhead attach points.                                 | `1.Load_Assy(PAXSTR-A01)<br>2.Detail_Floor_Beam_Grid<br>3.Show_Sidewall_Attach_Points<br>4.Specify_Seat_Track_Layout<br>5.Populate_Title_Block(PAXSTR_Dwg)`                       |
| 4     | 1.3.2.1  | `STR-FLRGRD-A01-SXXXX`                            | Floor Grid Structure                        | 1   | EA   | Al Alloy Extrusions/CFRP      | 17000x7000x150 (Aprox.)             | Est. 400 kg                     | Part of Pax Cabin Structure. `MDL: FLRGRD-A01`. Supports `A25` floor panels.                   | `1.Load_Model(FLRGRD-A01)<br>2.Show_Beam_XSections_Joints<br>3.Detail_Seat_Track_Integration<br>4.Specify_Load_Capacity<br>5.Populate_Title_Block(FLRGRD-A01_Dwg)`                  |
| 4     | 1.3.2.7  | `A25-SEATTRK-A01-SXXXX`                           | Seat Track System (Structural Rails)        | 1   | SET  | High-Strength Al Alloy        | Varies per rail (e.g. 5000x80x40)   | ~2800 kg/m³                     | SSS: `A25` (Equip/Furnish) as it's tied to seat install. `MDL: SEATTRK-A01`.                   | `1.Load_Profile(SEATTRK-A01)<br>2.Define_XSection_View<br>3.Specify_Cut_Lengths_Table<br>4.Detail_Attachment_Holes<br>5.Populate_Title_Block(SEATTRK-A01_Dwg)`                      |
| 2     | 1.4      | `A52-DOORGRP-D001-S0001`                          | Doors & Hatches Assembly (Group)            | 1   | EA   | Various                       | N/A (Group)                         | Est. 500 kg (Total for group)   | SSS: `A52` (Doors). Individual doors below are MDLs within A52.                                 | `1.Create_Door_Schedule_Dwg(DOORGRP-D001)<br>2.Ref_Individual_Door_Dwgs<br>3.Show_Aircraft_Locations<br>4.General_Seal_Spec_Notes<br>5.Populate_Title_Block(DOORGRP_GA)`            |
| 3     | 1.4.1    | `A52-DOORPAX-T1L-SXXXX`                           | Passenger Door Assembly, Type 1 Left        | 1   | EA   | Al Alloy / Composite          | 1900x1100x200 (Aprox.)              | Est. 80 kg                      | `MDL: DOORPAX-T1L`. Incl. mechanism, seals.                                                     | `1.Load_Assy(DOORPAX-T1L)<br>2.Show_Mechanism_Kinematics<br>3.Detail_Seal_Interface<br>4.Specify_Emergency_Op_Notes<br>5.Populate_Title_Block(DOORPAX-T1L_Dwg)`                   |
| 3     | 1.4.2    | `A52-EXITHTCH-T3R-SXXXX`                          | Emergency Exit Hatch Assembly, Type 3 Right | 1   | EA   | Composite                     | 900x500x150 (Aprox.)                | Est. 25 kg                      | `MDL: EXITHTCH-T3R`. Incl. mechanism, seals.                                                    | `1.Load_Assy(EXITHTCH-T3R)<br>2.Show_Open_Close_Path<br>3.Detail_Latch_Mechanism<br>4.Specify_Material_Cert<br>5.Populate_Title_Block(EXITHTCH-T3R_Dwg)`                         |
| 3     | 1.4.4    | **`AXX-ACHAVN-F01-SXXXX`**                        | Avionics Bay Access Hatch Assembly (Fwd)    | 1   | EA   | Al Alloy                      | 800x600x50 (Aprox.)                 | Est. 15 kg                      | SSS: `AXX` (Avionics general access). `MDL: ACHAVN-F01`.                                        | `1.Load_Model(ACHAVN-F01)<br>2.Show_Latch_Seal_Details<br>3.Detail_Hinge_Mechanism<br>4.Specify_Grounding_Points<br>5.Populate_Title_Block(ACHAVN-F01_Dwg)`                       |
| 3     | 1.4.5    | **`QAB-ACHQNT-M01-SXXXX`**                        | Quantum Systems Bay Access Hatch Assembly (Mid) | 1   | EA   | Composite / Shielding Mat.    | 1000x800x80 (Aprox.)                | Est. 30 kg                      | SSS: `QAB` (Quantum Avionics Bay). `MDL: ACHQNT-M01`. Shielded.                                 | `1.Load_Model(ACHQNT-M01)<br>2.Detail_Shielding_Layers<br>3.Specify_Seal_Conductivity<br>4.Show_Latch_Points<br>5.Populate_Title_Block(ACHQNT-M01_Dwg)`                        |
| 2     | 1.5      | `A32-LNDGSTR-D001-S0001`                          | Landing Gear Integration Structures Assembly | 1   | EA   | High-Strength Steel / Ti      | N/A (Group of Structures)           | Est. 1200 kg (Total for group)  | SSS: `A32` (Landing Gear). Structures supporting the LG.                                        | `1.Create_LG_Struct_Integration_Dwg(LNDGSTR-D001)<br>2.Ref_Individual_Bay_Dwgs<br>3.Show_Trunnion_Load_Points<br>4.Overall_Material_Spec<br>5.Populate_Title_Block(LNDGSTR_GA)` |
| 3     | 1.5.1    | `A32-NLGSTR-BAY01-SXXXX`                          | NLG Bay Structure                        | 1   | EA   | CFRP / Al Alloy               | 2500x1500x1200 (Aprox.)             | Est. 150 kg                     | `MDL: NLGSTR-BAY01`. Incl. door attach points.                                                  | `1.Load_Model(NLGSTR-BAY01)<br>2.Define_Views_Showing_Interfaces<br>3.Detail_Door_Attach_Features<br>4.Specify_Drainage_Provisions<br>5.Populate_Title_Block(NLGSTR-BAY01_Dwg)`    |
| 3     | 1.5.2    | `A32-MLGSTR-BAYP-SXXXX`                           | MLG Bay Structure (Port)                 | 1   | EA   | CFRP / Ti Reinforcements      | 3500x2000x1500 (Aprox.)             | Est. 450 kg                     | `MDL: MLGSTR-BAYP`. Integrated with primary structure.                                          | `1.Load_Model(MLGSTR-BAYP)<br>2.Show_Spar_Rib_Integration<br>3.Detail_Trunnion_Mounts<br>4.Specify_Material_Buildup<br>5.Populate_Title_Block(MLGSTR-BAYP_Dwg)`                     |
| 3     | 1.5.3    | `A32-MLGSTR-BAYS-SXXXX`                           | MLG Bay Structure (Starboard)            | 1   | EA   | CFRP / Ti Reinforcements      | 3500x2000x1500 (Aprox.)             | Est. 450 kg                     | `MDL: MLGSTR-BAYS`. Integrated with primary structure.                                          | `1.Load_Model(MLGSTR-BAYS)<br>2.Show_Spar_Rib_Integration<br>3.Detail_Trunnion_Mounts<br>4.Specify_Material_Buildup<br>5.Populate_Title_Block(MLGSTR-BAYS_Dwg)`                     |
| 2     | 1.6      | `STR-SYSBAYGRP-D001-S0001`                        | Systems Integration Bays & Mounting Struct. Assy | 1   | EA   | Various                       | N/A (Aggregate Group)               | Est. 800 kg (Total for group)   | SSS: `STR-SYSBAYGRP`. General structure for housing system LRUs.                                | `1.Create_SysBay_Layout_Dwg(SYSBAYGRP-D001)<br>2.Ref_Individual_Bay_Struct_Dwgs<br>3.Show_Zone_Allocation<br>4.General_Fastener_Notes<br>5.Populate_Title_Block(SYSBAYGRP_GA)`    |
| 3     | 1.6.1    | **`AXX-AVBAYSTR-F01-SXXXX`**                      | General Avionics Bay Structure (Fwd)     | 1   | EA   | Al Alloy                      | 2000x1500x1000 (Aprox.)             | Est. 100 kg                     | SSS: `AXX` (Avionics general for bay structure). `MDL: AVBAYSTR-F01`. For LRUs under `A22, A34, A23` etc. | `1.Load_Model(AVBAYSTR-F01)<br>2.Define_Rack_Layout_Views<br>3.Detail_LRU_Mounting_Holes<br>4.Specify_Grounding_Studs<br>5.Populate_Title_Block(AVBAYSTR-F01_Dwg)`                  |
| 3     | 1.6.2    | **`QAB-MODSTR-Q01-SXXXX`**                        | Quantum Systems Bays Module Structure (Aggregate) | 1   | EA   | Composite / Shielding         | 2500x2000x1200 (Envolvente)         | Est. 250 kg                     | SSS: `QAB` (Quantum Avionics Bay structure). `MDL: MODSTR-Q01`. For QNS, QFO, QComms bays.      | `1.Load_Assy(MODSTR-Q01)<br>2.Show_Internal_Bay_Divisions<br>3.Detail_Overall_Shielding_Spec<br>4.Interface_Control_Drawing_Ref<br>5.Populate_Title_Block(MODSTR-Q01_Assy)`       |
| 4     | 1.6.2.1  | **`QNS-BAYSTR-A01-SXXXX`**                        | QNS Bay Structure                        | 1   | EA   | Mu-Metal / CFRP               | 800x600x500 (Aprox.)                | ~8700 kg/m³ (Mu-Metal Layer)    | SSS: `QNS` (Quantum Nav Sys for the bay struct). `MDL: BAYSTR-A01`. Houses QNS LRU.            | `1.Load_Model(QNS-BAYSTR-A01)<br>2.Detail_MuMetal_Layer_Spec<br>3.Show_Vibration_Iso_Mounts<br>4.Thermal_Interface_Points<br>5.Populate_Title_Block(QNS-BAYSTR-A01_Dwg)`         |
| 4     | 1.6.2.2  | **`QFO-BAYSTR-A01-SXXXX`**                        | QFO Processor Bay Structure              | 1   | EA   | Al Alloy / CFRP               | 700x500x400 (Aprox.)                | Est. 30 kg                      | SSS: `QFO` (Quantum Flight Opt for bay struct). `MDL: BAYSTR-A01`. Houses QFO LRU.           | `1.Load_Model(QFO-BAYSTR-A01)<br>2.Detail_Cooling_Channel_Interfaces<br>3.Show_Connector_Panel_Cutouts<br>4.EMI_Gasket_Spec<br>5.Populate_Title_Block(QFO-BAYSTR-A01_Dwg)`       |
| 4     | 1.6.2.3  | **`QCO-BAYSTR-A01-SXXXX`**                        | QComms Transceiver Bay Structure         | 1   | EA   | CFRP                          | 600x400x300 (Aprox.)                | Est. 20 kg                      | SSS: `QCO` (Quantum Comms for bay struct). `MDL: BAYSTR-A01`. Houses QCO LRU.                | `1.Load_Model(QCO-BAYSTR-A01)<br>2.Detail_Waveguide_Fiber_Entries<br>3.Show_Thermal_Dissipation_Path<br>4.Mounting_Hole_Pattern<br>5.Populate_Title_Block(QCO-BAYSTR-A01_Dwg)`     |
| 3     | 1.6.3    | `A21-BAYECS-PK1-SXXXX`                            | ECS Pack Bay Structure                   | 1   | EA   | Al Alloy                      | 1200x1000x800 (Aprox.)              | Est. 60 kg                      | SSS: `A21` (Air Conditioning). `MDL: BAYECS-PK1`. Mounting for ECS packs.                     | `1.Load_Model(BAYECS-PK1)<br>2.Show_Duct_Interfaces<br>3.Detail_Pack_Mounting_Rails<br>4.Specify_Drainage_Provisions<br>5.Populate_Title_Block(BAYECS-PK1_Dwg)`                   |
| 2     | 1.7      | `A71-ENGINTSTR-D001-S0001`                        | Engine Integration Structures Assembly   | 1   | EA   | Ti / High-Temp Composite      | N/A (Aggregate Group)               | Est. 1500 kg/engine side (Total) | SSS: `A71` (Power Plant). Structures for engine integration.                                    | `1.Create_Eng_Int_Struct_GA(ENGINTSTR-D001)<br>2.Ref_Mount_Firewall_Duct_Dwgs<br>3.Show_Engine_Envelope_Clearance<br>4.Material_Heat_Treat_Specs<br>5.Populate_Title_Block(ENGINTSTR_GA)`|
| 3     | 1.7.1    | `A71-MNTENG-P01-SXXXX`                            | Engine Mount Hardpoints/Pylon Interface (Port) | 1   | EA   | Ti Forging                    | 800x500x300 (Aprox.)                | ~4500 kg/m³                     | `MDL: MNTENG-P01`. Specific to engine type (e.g., `A72-ENGMDLTYPE-ENGSERIAL`).                 | `1.Load_Model(MNTENG-P01)<br>2.Define_Views_Showing_Load_Faces<br>3.Detail_Bolt_Hole_Patterns<br>4.Specify_Forging_Grain_Flow<br>5.Populate_Title_Block(MNTENG-P01_Dwg)`           |
| 3     | 1.7.2    | `A71-FWLENG-P01-SXXXX`                            | Firewall Assembly (Port Engine)          | 1   | EA   | Ti / Composite                | 1500x1200x50 (Aprox.)               | Est. 70 kg                      | `MDL: FWLENG-P01`.                                                                              | `1.Load_Model(FWLENG-P01)<br>2.Show_Material_Layers_Stackup<br>3.Detail_Seal_Interfaces<br>4.Specify_Fireproof_Cert_Ref<br>5.Populate_Title_Block(FWLENG-P01_Dwg)`                 |
| 3     | 1.7.4    | `A71-DUCTIN-P01-SXXXX`                            | Inlet Duct Structural Shell (Port, if embedded) | 1   | EA   | CFRP / Acoustic Liner         | 2000x1800x1500 (Aprox.)             | Est. 120 kg                     | `MDL: DUCTIN-P01`.                                                                              | `1.Load_Model(DUCTIN-P01)<br>2.Show_Internal_Aerodynamic_Contour<br>3.Detail_Acoustic_Liner_Attach<br>4.Specify_Bird_Strike_Resist_Ref<br>5.Populate_Title_Block(DUCTIN-P01_Dwg)`  |
| 2     | 1.8      | `A5X-CTRLSURFINT-D001-S0001`                      | Empennage/Control Surface Interface Struct. Assy | 1   | EA   | CFRP                          | N/A (Aggregate Group)               | Est. 300 kg (Total for group)   | SSS: `A5X` (e.g., `A55` Stabilizers / `A57` Wings for integrated surfaces).                    | `1.Create_Emp_Struct_GA(CTRLSURFINT-D001)<br>2.Ref_Individual_Surf_Mount_Dwgs<br>3.Show_Actuator_Interface_Points<br>4.Overall_Composite_Spec<br>5.Populate_Title_Block(CTRLSURFINT_GA)`|
| 3     | 1.8.1    | `A55-MNTSTB-VTP-SXXXX`                            | V-Tail Mounting Frame (Port, if applicable) | 1   | EA   | CFRP / Ti Fittings            | 2500x800x200 (Aprox.)               | Est. 60 kg                      | `MDL: MNTSTB-VTP`.                                                                              | `1.Load_Model(MNTSTB-VTP)<br>2.Define_Views_Showing_Attach_Lugs<br>3.Detail_Spar_Interface<br>4.Specify_Fastener_Schedule<br>5.Populate_Title_Block(MNTSTB-VTP_Dwg)`                 |

---

Este es el BOM completo y expandido (Rev C) en un único bloque de Markdown. He intentado mantener el formato lo más legible posible dentro de las limitaciones de Markdown para tablas anchas.
**Aircraft Top-Level Object ID (Tier 1):** `AS-M-PAX-BW-Q1H-00001` *(Illustrative Instance)*
**Main Assembly ID (Tier 2 Conceptual):** `AS-M-PAX-BW-Q1H-00001 :: STR-CBA-D001-S0001`
**Main Assembly Name:** Main Fuselage/Central Body Assembly - BWB-Q100Small
**Date:** YYYY-MM-DD
**Revision:** C


## Conceptual Product Breakdown Structure (PBS) / Assembly Tree

*   **1.0 `STR-CBA-D001-S0001` Main Fuselage/Central Body Assembly (Qty: 1 EA, Mat: Various)**
    *   1.1 `STR-SHL-D001-S0001` Outer Shell & Aerodynamic Surfaces Assembly (Qty: 1 EA, Mat: Various Composite)
        *   1.1.1 `STR-SHL-UPNLD01-SXXXX` Upper Surface Panel Assembly (e.g., Fwd Upper) (Qty: 1 EA, Mat: CFRP T800/M55J)
        *   1.1.2 `STR-SHL-LPNLD01-SXXXX` Lower Surface Panel Assembly (e.g., Center Lower) (Qty: 1 EA, Mat: CFRP T800/M55J)
        *   1.1.3 **`A30-LEDSA-C001-SXXXX`** Leading Edge Sub-Assembly (Central Section) (Qty: 1 EA, Mat: CFRP / Ti)
        *   1.1.4 **`A27-TESA-C001-SXXXX`** Trailing Edge Sub-Assembly (Central Section) (Qty: 1 EA, Mat: CFRP)
        *   1.1.5 **`QCO-ANTCONF-A01-SXXXX`** Quantum Communications Conformal Antenna Array Assy (Qty: 2 EA, Mat: Dielectric Composite/RF Mat.)
        *   1.1.6 `STR-FAIRWBR-P01-SXXXX` Wing-to-Body Fairing Assembly (Port) (Qty: 1 EA, Mat: CFRP)
        *   1.1.7 `STR-FAIRWBR-S01-SXXXX` Wing-to-Body Fairing Assembly (Starboard) (Qty: 1 EA, Mat: CFRP)
    *   1.2 `STR-CORESA-D001-S0001` Primary Structural Core Assembly (Qty: 1 EA, Mat: Various Composite/Metallic)
        *   1.2.1 `STR-SPARMC-P01-SXXXX` Main Spar Assembly (Center Carry-Through, Port) (Qty: 1 EA, Mat: CFRP / Ti Fittings)
        *   1.2.2 `STR-SPARMC-S01-SXXXX` Main Spar Assembly (Center Carry-Through, Stbd) (Qty: 1 EA, Mat: CFRP / Ti Fittings)
        *   1.2.3 `STR-RIBSET-C01-SXXXX` Rib Assembly Set (Frames FS_XXX to FS_YYY) (Qty: 1 SET, Mat: CFRP / Al Alloy)
        *   1.2.4 `STR-FRMSET-C01-SXXXX` Frame Assembly Set (Frames STA_AAA to STA_BBB) (Qty: 1 SET, Mat: CFRP / Al Alloy)
        *   1.2.5 `STR-GRIDST-C01-SXXXX` Stringer & Stiffener Set (Qty: 1 SET, Mat: CFRP / Al Alloy)
        *   1.2.6 `STR-FBHDPRS-A01-SXXXX` Forward Pressure Bulkhead Assembly (Qty: 1 EA, Mat: CFRP / Al Alloy Honeycomb)
        *   1.2.7 `STR-ABHDPRS-A01-SXXXX` Aft Pressure Bulkhead Assembly (Qty: 1 EA, Mat: CFRP / Al Alloy Honeycomb)
        *   1.2.8 `STR-BHDNPSET-A01-SXXXX` Non-Pressurized Bay Bulkhead Set (MLG, Q-Bays) (Qty: 1 SET, Mat: CFRP / Al Alloy)
        *   1.2.9 `STR-KEELBM-A01-SXXXX` Keel Beam Assembly (If applicable) (Qty: 1 EA, Mat: Ti / CFRP)
    *   1.3 `STR-CABSTR-D001-S0001` Cabin Section Structural Assembly (Qty: 1 EA, Mat: Various Composite/Metallic)
        *   1.3.1 `STR-CKPSTR-A01-SXXXX` Cockpit Module Structure Assembly (Qty: 1 EA, Mat: Al Alloy / CFRP)
            *   1.3.1.1 `STR-WFRM-A01-SXXXX` Windscreen Frame (Qty: 1 EA, Mat: Ti / Al Alloy)
        *   1.3.2 `STR-PAXSTR-A01-SXXXX` Passenger Cabin Structure Assembly (Qty: 1 EA, Mat: Al Alloy / CFRP)
            *   1.3.2.1 `STR-FLRGRD-A01-SXXXX` Floor Grid Structure (Qty: 1 EA, Mat: Al Alloy Extrusions/CFRP)
            *   1.3.2.7 `A25-SEATTRK-A01-SXXXX` Seat Track System (Structural Rails) (Qty: 1 SET, Mat: High-Strength Al Alloy)
    *   1.4 `A52-DOORGRP-D001-S0001` Doors & Hatches Assembly (Group) (Qty: 1 EA, Mat: Various)
        *   1.4.1 `A52-DOORPAX-T1L-SXXXX` Passenger Door Assembly, Type 1 Left (Qty: 1 EA, Mat: Al Alloy / Composite)
        *   1.4.2 `A52-EXITHTCH-T3R-SXXXX` Emergency Exit Hatch Assembly, Type 3 Right (Qty: 1 EA, Mat: Composite)
        *   1.4.4 **`AXX-ACHAVN-F01-SXXXX`** Avionics Bay Access Hatch Assembly (Fwd) (Qty: 1 EA, Mat: Al Alloy)
        *   1.4.5 **`QAB-ACHQNT-M01-SXXXX`** Quantum Systems Bay Access Hatch Assembly (Mid) (Qty: 1 EA, Mat: Composite / Shielding Mat.)
    *   1.5 `A32-LNDGSTR-D001-S0001` Landing Gear Integration Structures Assembly (Qty: 1 EA, Mat: High-Strength Steel / Ti)
        *   1.5.1 `A32-NLGSTR-BAY01-SXXXX` NLG Bay Structure (Qty: 1 EA, Mat: CFRP / Al Alloy)
        *   1.5.2 `A32-MLGSTR-BAYP-SXXXX` MLG Bay Structure (Port) (Qty: 1 EA, Mat: CFRP / Ti Reinforcements)
        *   1.5.3 `A32-MLGSTR-BAYS-SXXXX` MLG Bay Structure (Starboard) (Qty: 1 EA, Mat: CFRP / Ti Reinforcements)
    *   1.6 `STR-SYSBAYGRP-D001-S0001` Systems Integration Bays & Mounting Struct. Assy (Qty: 1 EA, Mat: Various)
        *   1.6.1 **`AXX-AVBAYSTR-F01-SXXXX`** General Avionics Bay Structure (Fwd) (Qty: 1 EA, Mat: Al Alloy)
        *   1.6.2 **`QAB-MODSTR-Q01-SXXXX`** Quantum Systems Bays Module Structure (Aggregate) (Qty: 1 EA, Mat: Composite / Shielding)
            *   1.6.2.1 **`QNS-BAYSTR-A01-SXXXX`** QNS Bay Structure (Qty: 1 EA, Mat: Mu-Metal / CFRP)
            *   1.6.2.2 **`QFO-BAYSTR-A01-SXXXX`** QFO Processor Bay Structure (Qty: 1 EA, Mat: Al Alloy / CFRP)
            *   1.6.2.3 **`QCO-BAYSTR-A01-SXXXX`** QComms Transceiver Bay Structure (Qty: 1 EA, Mat: CFRP)
        *   1.6.3 `A21-BAYECS-PK1-SXXXX` ECS Pack Bay Structure (Qty: 1 EA, Mat: Al Alloy)
    *   1.7 `A71-ENGINTSTR-D001-S0001` Engine Integration Structures Assembly (Qty: 1 EA, Mat: Ti / High-Temp Composite)
        *   1.7.1 `A71-MNTENG-P01-SXXXX` Engine Mount Hardpoints/Pylon Interface (Port) (Qty: 1 EA, Mat: Ti Forging)
        *   1.7.2 `A71-FWLENG-P01-SXXXX` Firewall Assembly (Port Engine) (Qty: 1 EA, Mat: Ti / Composite)
        *   1.7.4 `A71-DUCTIN-P01-SXXXX` Inlet Duct Structural Shell (Port, if embedded) (Qty: 1 EA, Mat: CFRP / Acoustic Liner)
    *   1.8 `A5X-CTRLSURFINT-D001-S0001` Empennage/Control Surface Interface Struct. Assy (Qty: 1 EA, Mat: CFRP)
        *   1.8.1 `A55-MNTSTB-VTP-SXXXX` V-Tail Mounting Frame (Port, if applicable) (Qty: 1 EA, Mat: CFRP / Ti Fittings)

---

He verificado la lista contra la tabla del BOM Rev C que generamos previamente (la que incluía dimensiones, peso y prompt-trains) y esta estructura de árbol incluye todos los ítems hasta el nivel de detalle que habíamos definido allí. Los ítems que estaban en negrita en la tabla para resaltar los refinamientos también están en negrita aquí para mantener esa indicación.

Si hay algún ítem específico que crees que falta o si la indentación no se muestra correctamente de tu lado, por favor házmelo saber. El objetivo era presentar la jerarquía completa del BOM Rev C en este formato de árbol.
